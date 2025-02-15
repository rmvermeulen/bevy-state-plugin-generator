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

#[derive(Debug, PartialEq, Clone)]
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
            StateNode::Singleton(name) | StateNode::Enum(name, _) | StateNode::List(name, _) => {
                name
            }
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
            ParseNode::List(name, children) => StateNode::list(name, map_children(children)),
        }
    }
}

pub trait StateTree {
    fn get_size(&self) -> usize;
}

impl StateTree for StateNode {
    fn get_size(&self) -> usize {
        match self {
            StateNode::Singleton(_) => 1,
            StateNode::Enum(_, children) | StateNode::List(_, children) => {
                children.iter().map(|child| child.get_size()).sum::<usize>() + 1
            }
        }
    }
}

impl StateTree for ParseNode<'_> {
    fn get_size(&self) -> usize {
        match self {
            ParseNode::Singleton(_) => 1,
            ParseNode::Enum(_, children) | ParseNode::List(_, children) => {
                children.iter().map(|child| child.get_size()).sum::<usize>() + 1
            }
        }
    }
}
