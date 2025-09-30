pub use insta::*;
pub use rstest::*;
pub use speculoos::prelude::*;

pub use crate::set_snapshot_suffix;

#[macro_export]
#[allow(missing_docs)]
macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}

pub mod node_data {
    use bevy_utils::default;

    use super::*;
    use crate::processing::NodeData;

    fn node<S: ToString>(name: S, index: usize, depth: usize, parent: Option<usize>) -> NodeData {
        let name = name.to_string();
        NodeData {
            index,
            parent,
            resolved_name: Some(name.clone()),
            name,
            depth,
            ..default()
        }
    }

    #[fixture]
    pub fn single_node() -> Vec<NodeData> {
        vec![node("RootState", 0, 0, None)]
    }

    #[fixture]
    pub fn nested_example() -> Vec<NodeData> {
        vec![
            node("RootState", 0, 0, None),
            node("Menu", 1, 1, Some(0)),
            node("Options", 2, 2, Some(1)),
            node("Audio", 3, 3, Some(2)),
            node("Video", 4, 3, Some(2)),
        ]
    }
}

pub mod parse_node {
    use super::*;
    use crate::parsing::ParseNode;

    #[fixture]
    pub fn enum_root_a() -> ParseNode<'static> {
        ParseNode::enumeration("Root", [ParseNode::singleton("A")])
    }

    #[fixture]
    pub fn enum_root_ab() -> ParseNode<'static> {
        ParseNode::enumeration(
            "Root",
            [ParseNode::singleton("A"), ParseNode::singleton("B")],
        )
    }

    #[fixture]
    pub fn enum_root_a_b() -> ParseNode<'static> {
        ParseNode::enumeration(
            "Root",
            [ParseNode::enumeration("A", [ParseNode::singleton("B")])],
        )
    }

    #[fixture]
    pub fn enum_root_a_b_up_c() -> ParseNode<'static> {
        ParseNode::enumeration(
            "Root",
            [
                ParseNode::enumeration("A", [ParseNode::singleton("B")]),
                ParseNode::singleton("C"),
            ],
        )
    }

    #[fixture]
    pub fn list_root_a() -> ParseNode<'static> {
        ParseNode::list("Root", [ParseNode::singleton("A")])
    }

    #[fixture]
    pub fn list_root_ab() -> ParseNode<'static> {
        ParseNode::list(
            "Root",
            [ParseNode::singleton("A"), ParseNode::singleton("B")],
        )
    }

    #[fixture]
    pub fn list_root_a_b() -> ParseNode<'static> {
        ParseNode::list("Root", [ParseNode::list("A", [ParseNode::singleton("B")])])
    }

    #[fixture]
    pub fn list_root_a_b_up_c() -> ParseNode<'static> {
        ParseNode::list(
            "Root",
            [
                ParseNode::list("A", [ParseNode::singleton("B")]),
                ParseNode::singleton("C"),
            ],
        )
    }

    #[fixture]
    pub fn nested_example() -> ParseNode<'static> {
        ParseNode::enumeration(
            "Menu",
            [
                ParseNode::singleton("Main"),
                ParseNode::enumeration(
                    "Options",
                    [
                        ParseNode::singleton("Graphics"),
                        ParseNode::singleton("Audio"),
                        ParseNode::singleton("Gameplay"),
                    ],
                ),
                ParseNode::enumeration(
                    "Game",
                    [ParseNode::singleton("Save"), ParseNode::singleton("Load")],
                ),
            ],
        )
    }
}
