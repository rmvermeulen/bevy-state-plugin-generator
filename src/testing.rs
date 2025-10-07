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
    use crate::parsing::Node;

    #[fixture]
    pub fn duplicate_names() -> Node<'static> {
        Node::list("Root", [Node::singleton("A"), Node::singleton("A")])
    }

    #[fixture]
    pub fn overlapping_names() -> Node<'static> {
        Node::list(
            "Root",
            [
                Node::enumeration("Root", [Node::singleton("A")]),
                Node::singleton("A"),
            ],
        )
    }

    #[fixture]
    pub fn enum_root_a() -> Node<'static> {
        Node::enumeration("Root", [Node::singleton("A")])
    }

    #[fixture]
    pub fn enum_root_ab() -> Node<'static> {
        Node::enumeration("Root", [Node::singleton("A"), Node::singleton("B")])
    }

    #[fixture]
    pub fn enum_root_a_b() -> Node<'static> {
        Node::enumeration("Root", [Node::enumeration("A", [Node::singleton("B")])])
    }

    #[fixture]
    pub fn enum_root_a_b_up_c() -> Node<'static> {
        Node::enumeration(
            "Root",
            [
                Node::enumeration("A", [Node::singleton("B")]),
                Node::singleton("C"),
            ],
        )
    }

    #[fixture]
    pub fn list_root_a() -> Node<'static> {
        Node::list("Root", [Node::singleton("A")])
    }

    #[fixture]
    pub fn list_root_ab() -> Node<'static> {
        Node::list("Root", [Node::singleton("A"), Node::singleton("B")])
    }

    #[fixture]
    pub fn list_root_a_b() -> Node<'static> {
        Node::list("Root", [Node::list("A", [Node::singleton("B")])])
    }

    #[fixture]
    pub fn list_root_a_b_up_c() -> Node<'static> {
        Node::list(
            "Root",
            [
                Node::list("A", [Node::singleton("B")]),
                Node::singleton("C"),
            ],
        )
    }

    #[fixture]
    pub fn nested_example() -> Node<'static> {
        Node::enumeration(
            "Menu",
            [
                Node::singleton("Main"),
                Node::comment("these are the options"),
                Node::enumeration(
                    "Options",
                    [
                        Node::singleton("Graphics"),
                        Node::singleton("Audio"),
                        Node::singleton("Gameplay"),
                    ],
                ),
                Node::enumeration("Game", [Node::singleton("Save"), Node::singleton("Load")]),
            ],
        )
    }
}
