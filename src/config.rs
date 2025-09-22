/// How state-names are determined
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

impl std::fmt::Display for NamingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Configuration for the generated plugin
#[derive(Clone, Copy, Debug)]
pub struct PluginConfig<'a> {
    /// name of the struct that implements [`bevy::plugin::Plugin`]
    pub plugin_name: &'a str,
    /// name of the root enum/struct that represents the game state
    pub state_name: &'a str,
    /// name of the module that contains sub-states
    pub states_module_name: &'a str,
    /// naming scheme for the generated states
    pub naming_scheme: NamingScheme,
    /// add additional traits to the derive list
    pub additional_derives: &'a [&'a str],
}

/// Default configuration for the generated plugin
/// ```rust
/// # use bevy_state_plugin_generator::{NamingScheme, PluginConfig};
/// let config = PluginConfig::default();
/// assert_eq!(config.plugin_name, "GeneratedStatesPlugin");
/// assert_eq!(config.state_name, "GameState");
/// assert_eq!(config.states_module_name, "states");
/// assert_eq!(config.naming_scheme, NamingScheme::Full);
/// ```
impl Default for PluginConfig<'_> {
    fn default() -> Self {
        Self {
            plugin_name: "GeneratedStatesPlugin",
            state_name: "GameState",
            states_module_name: "states",
            naming_scheme: Default::default(),
            additional_derives: &[],
        }
    }
}

impl From<NamingScheme> for PluginConfig<'_> {
    fn from(naming_scheme: NamingScheme) -> Self {
        Self {
            naming_scheme,
            ..Default::default()
        }
    }
}
