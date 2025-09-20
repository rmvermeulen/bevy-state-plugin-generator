use itertools::Itertools;
use std::rc::Rc;

use crate::tokens::ParseNode;

#[derive(Clone, Debug, PartialEq)]
pub struct ParentState {
    pub name: String,
    pub variant: String,
}

impl ParentState {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn name_and_variant(&self) -> String {
        format!("{}::{}", self.name, self.variant)
    }
}

impl<N: ToString, V: ToString> From<(N, V)> for ParentState {
    fn from((name, variant): (N, V)) -> Self {
        Self {
            name: name.to_string(),
            variant: variant.to_string(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum StateNode {
    Singleton(String),
    Enum(String, Vec<Rc<StateNode>>),
    List(String, Vec<Rc<StateNode>>),
}

impl StateNode {
    pub fn singleton<S: ToString>(name: S) -> Self {
        Self::Singleton(name.to_string())
    }
    pub fn enumeration<N: Into<Rc<StateNode>>, V: IntoIterator<Item = N>, S: ToString>(
        name: S,
        variants: V,
    ) -> Self {
        Self::Enum(
            name.to_string(),
            variants.into_iter().map(Into::into).collect_vec(),
        )
    }
    #[cfg(test)]
    #[allow(dead_code)]
    pub fn enum_empty<S: ToString>(name: S) -> Self {
        Self::Enum(name.to_string(), vec![])
    }
    pub fn list<N: Into<Rc<StateNode>>, V: IntoIterator<Item = N>, S: ToString>(
        name: S,
        variants: V,
    ) -> Self {
        Self::List(
            name.to_string(),
            variants.into_iter().map(Into::into).collect_vec(),
        )
    }
    pub fn name(&self) -> &str {
        match self {
            StateNode::List(name, _) => name,
            StateNode::Singleton(name) | StateNode::Enum(name, _) => name,
        }
    }
}

impl std::fmt::Debug for StateNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn format_children(children: &[Rc<StateNode>]) -> String {
            children.iter().map(|c| format!("{c:?}")).join(", ")
        }
        match self {
            StateNode::Singleton(name) => write!(f, "{name}"),
            StateNode::Enum(name, children) => {
                write!(f, "{} {{ {} }}", name, format_children(children))
            }
            StateNode::List(name, children) => {
                write!(f, "{} [ {} ]", name, format_children(children))
            }
        }
    }
}

impl TryFrom<ParseNode<'_>> for StateNode {
    type Error = ();
    fn try_from(node: ParseNode) -> Result<Self, Self::Error> {
        let map_children = |children: Vec<ParseNode>| {
            children
                .into_iter()
                .flat_map(StateNode::try_from)
                .map(Rc::new)
                .collect_vec()
        };
        match node {
            ParseNode::Singleton(name) => Ok(StateNode::singleton(name)),
            ParseNode::Enum(name, children) => {
                Ok(StateNode::enumeration(name, map_children(children)))
            }
            ParseNode::List(name, children) => Ok(StateNode::list(name, map_children(children))),
            ParseNode::Comment(_) => Err(()),
        }
    }
}

pub trait SubTree {
    fn get_tree_size(&self) -> usize;
}

impl SubTree for StateNode {
    fn get_tree_size(&self) -> usize {
        match self {
            StateNode::Singleton(_) => 1,
            StateNode::Enum(_, children) => {
                children
                    .iter()
                    .map(|child| child.get_tree_size())
                    .sum::<usize>()
                    + 1
            }
            StateNode::List(_, children) => {
                children
                    .iter()
                    .map(|child| child.get_tree_size())
                    .sum::<usize>()
                    + 1
            }
        }
    }
}

impl SubTree for ParseNode<'_> {
    fn get_tree_size(&self) -> usize {
        match self {
            ParseNode::Comment(_) => 1,
            ParseNode::Singleton(_) => 1,
            ParseNode::Enum(_, children) => {
                children
                    .iter()
                    .map(|child| child.get_tree_size())
                    .sum::<usize>()
                    + 1
            }
            ParseNode::List(_, children) => {
                children
                    .iter()
                    .map(|child| child.get_tree_size())
                    .sum::<usize>()
                    + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::StateNode;
    use crate::testing::*;
    use crate::tokens::ParseNode;

    #[rstest]
    #[case("Main", ParseNode::singleton("Main"))]
    #[case("Main{}", ParseNode::Enum("Main".into(), Default::default()))]
    #[case("Main{A}", ParseNode::enumeration("Main", [ParseNode::singleton("A")]))]
    #[case("Main{A,B}", ParseNode::enumeration("Main", [
        ParseNode::singleton("A"),
        ParseNode::singleton("B"),
    ]))]
    fn test_parse_node_try_from_str(#[case] input: &str, #[case] expected: ParseNode) {
        assert_that!(ParseNode::try_from(input)).is_ok_containing(expected);
    }

    #[rstest]
    #[case(StateNode::singleton("A"), "A")]
    #[case(StateNode::enumeration("A", [StateNode::singleton("B")]), "A { B }")]
    #[case(StateNode::list("A", [StateNode::singleton("B")]), "A [ B ]")]
    fn test_state_node_debug(#[case] node: StateNode, #[case] expected: &str) {
        assert_eq!(format!("{node:?}"), expected);
    }
}
