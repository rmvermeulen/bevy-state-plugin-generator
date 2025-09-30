#[cfg(test)]
mod tests;

use std::collections::VecDeque;

use bevy_platform::collections::HashSet;
use bevy_utils::default;
use indoc::formatdoc;
use itertools::{Itertools, concat};

use crate::generate::REQUIRED_DERIVES;
use crate::parsing::ParseNode;
use crate::tree::SubTree;
use crate::{NamingScheme, PluginConfig, PluginName};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum NodeType {
    #[default]
    Singleton,
    List,
    Enum,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NodeData {
    pub index: usize,
    pub parent: Option<usize>,
    pub node_type: NodeType,
    pub depth: usize,
    pub name: String,
    pub resolved_name: Option<String>,
    pub variants: Vec<String>,
}

pub fn flatten_parse_node(root_node: ParseNode<'_>) -> Vec<NodeData> {
    let node_count = root_node.get_tree_size();
    let mut nodes = Vec::with_capacity(node_count);
    let mut todo = VecDeque::from([(root_node, 0, None)]);
    while let Some((parse_node, depth, parent)) = todo.pop_front() {
        let Some(name) = parse_node.name() else {
            continue;
        };
        let node_type = match parse_node {
            ParseNode::Singleton(_) => NodeType::Singleton,
            ParseNode::Enum(_, _) => NodeType::Enum,
            ParseNode::List(_, _) => NodeType::List,
            ParseNode::Comment(_) => {
                unreachable!("Comment has no name")
            }
        };
        let index = nodes.len();
        nodes.push(NodeData {
            node_type,
            index,
            parent,
            depth,
            name: name.to_string(),
            ..default()
        });
        for child in parse_node.children() {
            todo.push_back((child, depth + 1, Some(index)));
        }
    }

    for (i, node) in nodes.iter().enumerate() {
        assert_eq!(node.index, i);
        if let Some(parent) = node.parent {
            assert!(parent <= node.index);
            assert!(node.depth > nodes[parent].depth);
        }
    }

    nodes
}

pub fn apply_naming_scheme(
    naming_scheme: NamingScheme,
    nodes: &mut [NodeData],
) -> Result<(), ProcessingError> {
    let mut names = HashSet::new();
    let mut resolved_names = Vec::new();
    for node in nodes.iter() {
        let base_name = &node.name;
        let resolved_name = match naming_scheme {
            NamingScheme::Full => {
                let mut ancestors = Vec::new();
                let mut current = node;
                while let Some(p) = current.parent {
                    current = &nodes[p];
                    ancestors.push(current);
                }
                let ancestral_name = ancestors.into_iter().map(|a| &a.name).join("");
                format!("{ancestral_name}{base_name}")
            }
            NamingScheme::Short => {
                let parent = node
                    .parent
                    .map(|p| nodes[p].name.as_str())
                    .unwrap_or_default();
                format!("{parent}{base_name}")
            }
            NamingScheme::None => base_name.clone(),
        };
        if !names.insert(resolved_name.clone()) {
            return Err(ProcessingError::DuplicateName {
                resolved_name,
                original_name: base_name.clone(),
            });
        }

        resolved_names.push(resolved_name);
    }
    assert_eq!(names.len(), resolved_names.len());
    for (i, resolved_name) in resolved_names.into_iter().enumerate() {
        nodes[i].resolved_name = Some(resolved_name);
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("Duplicate name: resolved_name='{resolved_name}' original_name='{original_name}'")]
    DuplicateName {
        resolved_name: String,
        original_name: String,
    },
    #[error("Unspecified error: {0}")]
    Custom(String),
    #[error("Failed to parse! Final state: {0:?}")]
    Parsing(String),
}

fn get_source(nodes: Vec<NodeData>, config: PluginConfig) -> Result<String, ProcessingError> {
    let PluginConfig {
        additional_derives: derives,
        plugin_name,
        root_state_name,
        naming_scheme: _,
        states_module_name,
    } = config;

    // add the implicit root_node according to config
    let nodes = if let Some(root_state_name) = &root_state_name {
        let root_node = NodeData {
            index: 0,
            parent: None,
            name: root_state_name.clone(),
            resolved_name: Some(root_state_name.clone()),
            ..default()
        };
        let mut new_nodes = vec![root_node];
        for mut node in nodes.into_iter() {
            let root_node = &mut new_nodes[0];
            if node.depth == 0 {
                root_node.variants.push(
                    node.resolved_name
                        .clone()
                        .unwrap_or_else(|| panic!("Node name has not been resolved! {node:?}")),
                );
            }
            // adjust indices, since we've added a root_node
            node.index += 1;
            node.depth += 1;
            if let Some(parent_index) = &mut node.parent {
                *parent_index += 1;
            } else {
                // parent this node to the root_node
                _ = node.parent.insert(0);
            }
            assert!(node.parent.is_some());
            new_nodes.push(node);
        }
        new_nodes
    } else {
        nodes
    };

    let derives = concat([
        REQUIRED_DERIVES
            .iter()
            .map(ToString::to_string)
            .collect_vec(),
        derives.into_iter().collect_vec(),
    ])
    .into_iter()
    .unique()
    .join(", ");
    let definitions_source = nodes
        .iter()
        .map(|node| {
            let derives = node
                .parent
                .map(|parent_id| {
                    let parent = &nodes[parent_id];
                    formatdoc! {"
                        #[derive(bevy::prelude::SubStates, {derives})]
                        #[source({source} = {source}::{variant})]
                    ", variant = node.name, source = parent.name
                    }
                })
                .unwrap_or_else(|| formatdoc! {"#[derive(bevy::prelude::States, {derives})]"})
                .trim()
                .to_string();
            let typename = &node.name;

            let source_for_singleton = || {
                formatdoc! {"
                {derives}
                pub struct {typename};
            "}
            };
            let source_for_enum = |variants: &[&str]| {
                formatdoc! {"
                {derives}
                pub enum {typename} {{
                    #[default]
                    {variants}
                }}
                ", variants = variants.join(",\n      ")
                }
            };

            match node.node_type {
                NodeType::Singleton => source_for_singleton(),
                NodeType::Enum => {
                    if node.variants.is_empty() {
                        source_for_singleton()
                    } else {
                        let variants = node.variants.iter().map(String::as_str).collect_vec();
                        source_for_enum(&variants)
                    }
                }
                NodeType::List => source_for_singleton(),
            }
        })
        .join("\n");

    let plugin_builder = if let Some(root_state_name) = root_state_name.as_ref() {
        let states_module_name = states_module_name.as_str();
        let init_state = format!(".init_state::<{states_module_name}::{root_state_name}>()");
        let sub_states = nodes
            .iter()
            .flat_map(|node| {
                let resolved_name = node
                    .resolved_name
                    .as_ref()
                    .expect("Node name has not been resolved!");
                if node.parent.map(|p| nodes[p].clone()).is_some() {
                    Some(format!(
                        ".add_sub_state::<{states_module_name}::{resolved_name}>()"
                    ))
                } else {
                    None
                }
            })
            .join("\n            ");
        format!("app{init_state}{sub_states};")
    } else {
        let states_module_name = states_module_name.as_str();
        let states = nodes
            .iter()
            .map(|node| {
                let resolved_name = node
                    .resolved_name
                    .clone()
                    .expect("Node name has not been resolved!");
                if node.parent.map(|p| nodes[p].clone()).is_some() {
                    format!(".add_sub_state::<{states_module_name}::{resolved_name}>()")
                } else {
                    format!(".add_state::<{states_module_name}::{resolved_name}>()")
                }
            })
            .join("\n            ");
        format!("app{states};")
    };

    let plugin_def = match plugin_name {
        PluginName::Struct(plugin_name) => {
            formatdoc! {"
                pub struct {plugin_name};
                impl bevy::app::Plugin for {plugin_name} {{
                    fn build(&self, app: &mut bevy::app::App) {{
                        {plugin_builder}
                    }}
                }}
            "}
        }
        PluginName::Function(plugin_name) => {
            formatdoc! {"
                pub fn {plugin_name}(app: &mut bevy::app::App) {{
                    {plugin_builder}
                }}
            "}
        }
    };

    Ok(formatdoc! {"
        #![allow(missing_docs)]
        use bevy::prelude::AppExtStates;
        pub mod {states_module_name} {{
            use bevy::prelude::StateSet;
            {definitions_source}
        }}
        {plugin_def}
    "})
}

pub fn parse_node_into_final_source(
    node: ParseNode<'_>,
    config: PluginConfig,
) -> Result<String, ProcessingError> {
    let mut nodes = flatten_parse_node(node);
    apply_naming_scheme(config.naming_scheme, &mut nodes)?;
    assert!(nodes.iter().all(|node| node.resolved_name.is_some()));
    get_source(nodes, config)
}
