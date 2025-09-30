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

pub mod parse_node {
    use super::*;
    use crate::parsing::ParseNode;

    #[fixture]
    #[once]
    pub fn enum_root_a() -> ParseNode<'static> {
        ParseNode::enumeration("Root", [ParseNode::singleton("A")])
    }

    #[fixture]
    #[once]
    pub fn enum_root_ab() -> ParseNode<'static> {
        ParseNode::enumeration(
            "Root",
            [ParseNode::singleton("A"), ParseNode::singleton("B")],
        )
    }

    #[fixture]
    #[once]
    pub fn enum_root_a_b() -> ParseNode<'static> {
        ParseNode::enumeration(
            "Root",
            [ParseNode::enumeration("A", [ParseNode::singleton("B")])],
        )
    }

    #[fixture]
    #[once]
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
    #[once]
    pub fn list_root_a() -> ParseNode<'static> {
        ParseNode::list("Root", [ParseNode::singleton("A")])
    }

    #[fixture]
    #[once]
    pub fn list_root_ab() -> ParseNode<'static> {
        ParseNode::list(
            "Root",
            [ParseNode::singleton("A"), ParseNode::singleton("B")],
        )
    }

    #[fixture]
    #[once]
    pub fn list_root_a_b() -> ParseNode<'static> {
        ParseNode::list("Root", [ParseNode::list("A", [ParseNode::singleton("B")])])
    }

    #[fixture]
    #[once]
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
    #[once]
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
                    "Continue",
                    [ParseNode::singleton("Save"), ParseNode::singleton("Load")],
                ),
            ],
        )
    }
}
