#[cfg(test)]
mod tests;

use std::collections::VecDeque;

use bevy_platform::collections::HashSet;

use crate::parsing::ParseNode;
use crate::tree::SubTree;
use crate::NamingScheme;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NodeType {
    Singleton,
    List,
    Enum,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NodeData {
    index: usize,
    parent: Option<usize>,
    node_type: NodeType,
    depth: usize,
    name: String,
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

fn apply_naming_scheme(naming_scheme: NamingScheme, nodes: Vec<NodeData>) {
    let mut names = HashSet::new();
    for node in &nodes {
        let base_name = &node.name;
        let resolved_name = match naming_scheme {
            NamingScheme::Full => todo!(),
            NamingScheme::Short => todo!(),
            NamingScheme::None => base_name.clone(),
        };
        assert!(
            names.insert(resolved_name.clone()),
            "Duplicate name {:?}",
            &node.name
        );
    }
}
