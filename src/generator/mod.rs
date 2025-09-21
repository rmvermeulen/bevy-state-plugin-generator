mod context;
mod models;
#[cfg(test)]
mod tests;

use std::rc::Rc;

use context::Context;
use indoc::formatdoc;
use itertools::Itertools;
use models::{TypeDef, TypeDefinitions};
use nom::AsChar;

use crate::models::{ParentState, StateNode, SubTree};
use crate::parsing::parse_states_text;
use crate::{NamingScheme, PluginConfig};

pub(super) const REQUIRED_DERIVES: &[&str] =
    &["Hash", "Default", "Debug", "Clone", "PartialEq", "Eq"];

pub trait ToStringWith {
    fn to_string_indented<S: AsRef<str>>(&self, join: S) -> String;
}

pub trait ToStateName {
    fn to_state_name(&self) -> String;
}

impl<S: ToString> ToStateName for S {
    fn to_state_name(&self) -> String {
        let s = self.to_string();
        if s.ends_with("State") {
            s
        } else {
            format!("{s}State")
        }
    }
}

fn get_typedef(
    node: &StateNode,
    Context {
        naming_scheme,
        parent_state,
        derives,
    }: Context,
) -> TypeDef {
    let derives = parent_state
        .clone()
        .map(|parent_state| {
            let source = parent_state.name();
            let variant = parent_state.name_and_variant();
            formatdoc! {"
                #[derive(bevy::prelude::SubStates, {derives})]
                #[source({source} = {variant})]
            "}
        })
        .unwrap_or_else(|| formatdoc! {"#[derive(bevy::prelude::States, {derives})]"})
        .trim()
        .to_string();
    let typename = if naming_scheme == NamingScheme::None {
        node.name().to_state_name()
    } else {
        parent_state
            .clone()
            .map(|parent_state| {
                let parent_name = parent_state
                    .name()
                    .strip_suffix("State")
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| parent_state.name());
                assert!(!parent_name.is_empty());
                let child_name = node
                    .name()
                    .strip_suffix("State")
                    .unwrap_or_else(|| node.name());
                assert!(!child_name.is_empty());
                format!("{parent_name}{child_name}")
            })
            .unwrap_or_else(|| node.name().to_string())
            .to_state_name()
    };
    let source_for_struct = || {
        formatdoc! {"
            {derives}
            pub struct {typename};
        "}
    };
    let source_for_enum = |variants: &Vec<Rc<StateNode>>| {
        let variants = variants
            .iter()
            .map(|variant| variant.name())
            .join(",\n      ");
        formatdoc! {"
            {derives}
            pub enum {typename} {{
                #[default]
                {variants}
            }}
        "}
    };
    match node {
        StateNode::List(_, _) => TypeDef {
            source: source_for_struct(),
            typename,
        },
        StateNode::Singleton(_) => TypeDef {
            source: source_for_struct(),
            typename,
        },
        StateNode::Enum(_, variants) => TypeDef {
            source: if variants.is_empty() {
                source_for_struct()
            } else {
                source_for_enum(variants)
            },
            typename,
        },
    }
}

fn generate_all_type_definitions(root_node: &StateNode, context: Context) -> TypeDefinitions {
    let root_typedef = get_typedef(root_node, context.clone());
    match root_node {
        StateNode::Singleton(_) => vec![root_typedef].into(),
        StateNode::Enum(_, variants) => {
            let mut variants = variants
                .iter()
                .flat_map(|child_node| {
                    let parent_state = ParentState::new(
                        match context.naming_scheme {
                            NamingScheme::Short | NamingScheme::None => {
                                root_node.name().to_string()
                            }
                            NamingScheme::Full => root_typedef.typename.clone(),
                        },
                        child_node.name(),
                    );
                    generate_all_type_definitions(
                        child_node,
                        Context {
                            parent_state: Some(parent_state),
                            derives: context.derives.clone(),
                            naming_scheme: context.naming_scheme,
                        },
                    )
                    .take()
                })
                .collect_vec();
            {
                let mut typedefs = vec![root_typedef];
                typedefs.append(&mut variants);
                typedefs.into()
            }
        }
        StateNode::List(_, variants) => {
            let mut variants = variants
                .iter()
                .flat_map({
                    |child_node| {
                        // NOTE: pass along current Context since List does not actually render
                        // into a struct, but refers to its parent
                        generate_all_type_definitions(child_node, context.clone()).take()
                    }
                })
                .collect_vec();
            {
                let mut typedefs = vec![root_typedef];
                typedefs.append(&mut variants);
                typedefs.into()
            }
        }
    }
}

pub fn generate_debug_info(src_path: &str, source: &str) -> String {
    let pkg = env!("CARGO_PKG_NAME");
    #[cfg(not(test))]
    let version = env!("CARGO_PKG_VERSION");
    #[cfg(test)]
    let version = "CARGO_PKG_VERSION";
    let lines = source.lines().map(|line| format!("// {line}")).join("\n");
    formatdoc! {"
        // generated by {pkg} v{version}
        // src: {src_path}
        {lines}
    "}
}

pub(crate) fn generate_plugin_source(root_state: Rc<StateNode>, config: PluginConfig) -> String {
    let PluginConfig {
        plugin_name,
        state_name,
        states_module_name,
        scheme: _,
        additional_derives,
    } = config;

    let context = Context::from(config.scheme);
    let context = if additional_derives.is_empty() {
        context
    } else {
        Context {
            derives: [context.derives, additional_derives.join(", ")].join(", "),
            ..context
        }
    };

    let type_definitions = generate_all_type_definitions(&root_state, context);
    let definitions_source = type_definitions.to_string_indented("    ");
    let sub_states = type_definitions
        .take()
        .into_iter()
        .skip(1) // skip the root
        .map(|typedef| typedef.typename)
        .map(|state_name| format!(".add_sub_state::<{states_module_name}::{state_name}>()"))
        .join("\n            ");
    formatdoc! {"
        #![allow(missing_docs)]
        use bevy::prelude::AppExtStates;
        pub mod {states_module_name} {{
            use bevy::prelude::StateSet;
            {definitions_source}
        }}
        pub struct {plugin_name};
        impl bevy::app::Plugin for {plugin_name} {{
            fn build(&self, app: &mut bevy::app::App) {{
                app.init_state::<{states_module_name}::{state_name}>()
                    {sub_states}
                ;
            }}
        }}
    "}
}

#[cfg(feature = "rustfmt")]
pub fn try_format_source(source: &str) -> std::io::Result<String> {
    duct::cmd!("rustfmt")
        .stdin_bytes(source)
        .stderr_to_stdout()
        .read()
}

pub fn format_source<S: AsRef<str>>(source: S) -> String {
    let source = source.as_ref();
    #[cfg(feature = "rustfmt")]
    let source = try_format_source(source).unwrap_or_else(|_| source.to_owned());
    #[cfg(not(feature = "rustfmt"))]
    let source = source.to_owned();

    if source.ends_with(|c: char| c.is_newline()) {
        source
    } else {
        source + "\n"
    }
}

pub fn generate_state_plugin_source(
    source: &str,
    plugin_config: PluginConfig,
    src_path: Option<&str>,
) -> Result<String, String> {
    let parse_tree =
        parse_states_text(source, plugin_config.state_name).map_err(|e| e.to_string())?;

    let parse_tree_size = parse_tree.get_tree_size();

    let root_node: Rc<StateNode> = parse_tree
        .try_into()
        .map(Rc::new)
        .map_err(|e| format!("{e:?}"))?;
    let state_tree_size = root_node.get_tree_size();

    if state_tree_size > parse_tree_size {
        return Err("state-tree exceeds parse-tree!".into());
    }

    let plugin_source = generate_plugin_source(root_node, plugin_config);
    let source = if let Some(src_path) = src_path {
        let debug_info = generate_debug_info(src_path, source);
        [debug_info, plugin_source].join("\n")
    } else {
        plugin_source
    };

    Ok(format_source(source))
}
