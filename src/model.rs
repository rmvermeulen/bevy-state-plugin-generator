use derive_more::From;
use iter_tools::Itertools;
use std::rc::Rc;

use crate::tokens::ParseNode;

#[derive(PartialEq, Debug, Clone)]
pub struct SourceState {
    pub name: String,
    pub variant: String,
}

impl SourceState {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn name_and_variant(&self) -> String {
        format!("{}::{}", self.name, self.variant)
    }
}

/// How state-names are determined
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamingScheme {
    /// Name includes the names of all ancestors
    #[default]
    Full,
    /// Name includes only the name of the immediate parent
    Short,
    /// None (all names must be unique)
    None,
}

impl NamingScheme {
    /// Get the name of the naming scheme
    /// ```rust
    /// # use bevy_state_plugin_generator::NamingScheme;
    /// assert_eq!(NamingScheme::Full.name(), "Full");
    /// assert_eq!(NamingScheme::Short.name(), "Short");
    /// assert_eq!(NamingScheme::None.name(), "None");
    /// ```
    pub fn name(&self) -> &str {
        match self {
            NamingScheme::Full => "Full",
            NamingScheme::Short => "Short",
            NamingScheme::None => "None",
        }
    }
    /// Get the identifying tag for this scheme
    /// ```rust
    /// # use bevy_state_plugin_generator::NamingScheme;
    /// assert_eq!(NamingScheme::Full.tag(), "full");
    /// assert_eq!(NamingScheme::Short.tag(), "short");
    /// assert_eq!(NamingScheme::None.tag(), "none");
    /// ```
    pub fn tag(&self) -> String {
        self.name().to_lowercase()
    }
}

/// Configuration for the generated plugin
#[derive(Debug, Clone, Copy)]
pub struct PluginConfig<'a> {
    /// name of the struct that implements [bevy::plugin::Plugin]
    pub plugin_name: &'a str,
    /// name of the root enum/struct that represents the game state
    pub state_name: &'a str,
    /// name of the module that contains sub-states
    pub states_module_name: &'a str,
    /// naming scheme for the generated states
    pub scheme: NamingScheme,
}

/// Default configuration for the generated plugin
/// ```rust
/// # use bevy_state_plugin_generator::{NamingScheme, PluginConfig};
/// let config = PluginConfig::default();
/// assert_eq!(config.plugin_name, "GeneratedStatesPlugin");
/// assert_eq!(config.state_name, "GameState");
/// assert_eq!(config.states_module_name, "states");
/// assert_eq!(config.scheme, NamingScheme::Full);
/// ```
impl Default for PluginConfig<'_> {
    fn default() -> Self {
        Self {
            plugin_name: "GeneratedStatesPlugin",
            state_name: "GameState",
            states_module_name: "states",
            scheme: Default::default(),
        }
    }
}

impl From<NamingScheme> for PluginConfig<'_> {
    fn from(scheme: NamingScheme) -> Self {
        Self {
            scheme,
            ..Default::default()
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum StateNode {
    Singleton(String),
    Enum(String, Vec<Rc<StateNode>>),
    #[cfg(feature = "lists")]
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
    #[cfg(feature = "lists")]
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
            #[cfg(feature = "lists")]
            StateNode::List(name, _) => name,
            StateNode::Singleton(name) | StateNode::Enum(name, _) => name,
        }
    }
}

impl std::fmt::Debug for StateNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateNode::Singleton(name) => write!(f, "{}", name),
            StateNode::Enum(name, children) => write!(f, "{} {{ {:?} }}", name, children),
            #[cfg(feature = "lists")]
            StateNode::List(name, children) => write!(f, "{} [ {:?} ]", name, children),
        }
    }
}

impl From<ParseNode<'_>> for StateNode {
    fn from(node: ParseNode) -> Self {
        let map_children = |children: Vec<ParseNode>| {
            children
                .into_iter()
                .map(Into::into)
                .map(Rc::new)
                .collect_vec()
        };
        match node {
            ParseNode::Singleton(name) => StateNode::singleton(name),
            ParseNode::Enum(name, children) => StateNode::enumeration(name, map_children(children)),
            #[cfg(feature = "lists")]
            ParseNode::List(name, children) => StateNode::list(name, map_children(children)),
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
            #[cfg(feature = "lists")]
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
            ParseNode::Singleton(_) => 1,
            ParseNode::Enum(_, children) => {
                children
                    .iter()
                    .map(|child| child.get_tree_size())
                    .sum::<usize>()
                    + 1
            }
            #[cfg(feature = "lists")]
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

#[derive(PartialEq, Clone, From)]
pub struct StateTree {
    root: Rc<StateNode>,
}

impl StateTree {
    pub fn create<N: Into<StateNode>, I: IntoIterator<Item = N>>(nodes: I) -> Self {
        Self {
            root: Rc::new(StateNode::enumeration(
                ":root:",
                nodes.into_iter().map(Into::into).map(Rc::new).collect_vec(),
            )),
        }
    }
    pub fn get_size(&self) -> usize {
        let subsize = self.root.get_tree_size();
        assert!(subsize > 0);
        subsize - 1
    }
}

impl std::fmt::Debug for StateTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StateTree {{ {:?} }}", self.root)
    }
}

impl SubTree for StateTree {
    fn get_tree_size(&self) -> usize {
        self.get_size()
    }
}
#[cfg(test)]
mod tests {
    use crate::model::{StateTree, SubTree};
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
    fn test_state_tree() {
        let a: ParseNode = "PartA".try_into().unwrap();
        let b: ParseNode = "PartB".try_into().unwrap();
        let tree = StateTree::create([a, b]);
        assert_that!(tree.get_tree_size()).is_equal_to(2);
        assert_debug_snapshot!(tree, @"StateTree { :root: { [PartA, PartB] } }");
    }
}
