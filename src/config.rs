/// How state-names are determined
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NamingScheme {
    /// Name includes the names of all ancestors
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
    /// Convert a string into a NamingScheme, if it's the name or tag
    pub fn try_parse(input: &str) -> Option<Self> {
        match input {
            "full" | "Full" => Some(NamingScheme::Full),
            "short" | "Short" => Some(NamingScheme::Short),
            "none" | "None" => Some(NamingScheme::None),
            _ => None,
        }
    }
}

impl Default for NamingScheme {
    fn default() -> Self {
        Self::Full
    }
}

impl std::fmt::Display for NamingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// How the plugin is rendered.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PluginName<'s> {
    /// ```rust
    /// use bevy_state_plugin_generator::config::PluginName;
    /// PluginName::Struct("MyPlugin");
    /// ```
    Struct(&'s str),
    /// ```rust
    /// use bevy_state_plugin_generator::config::PluginName;
    /// PluginName::Function("my_plugin");
    /// ```
    Function(&'s str),
}

/// Configuration for the generated plugin
#[derive(Clone, Debug)]
pub struct PluginConfig {
    /// name of the struct that implements [`bevy::plugin::Plugin`]
    pub plugin_name: PluginName<'static>,
    /// name of the root enum/struct that represents the game state
    pub root_state_name: Option<String>,
    /// name of the module that contains sub-states
    pub states_module_name: String,
    /// naming scheme for the generated states
    pub naming_scheme: NamingScheme,
    /// add additional traits to the derive list
    pub additional_derives: Vec<String>,
}

/// Default configuration for the generated plugin
/// ```rust
/// # use bevy_state_plugin_generator::*;
/// let config = PluginConfig::default();
/// assert_eq!(config.plugin_name, PluginName::Struct("GeneratedStatesPlugin"));
/// assert_eq!(config.root_state_name, Some("GameState".to_string()));
/// assert_eq!(config.states_module_name, "states".to_string());
/// assert_eq!(config.naming_scheme, NamingScheme::Full);
/// ```
impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            plugin_name: PluginName::Struct("GeneratedStatesPlugin"),
            root_state_name: Some("GameState".to_string()),
            states_module_name: "states".to_string(),
            naming_scheme: Default::default(),
            additional_derives: vec![],
        }
    }
}

impl From<NamingScheme> for PluginConfig {
    fn from(naming_scheme: NamingScheme) -> Self {
        Self {
            naming_scheme,
            ..Default::default()
        }
    }
}

#[cfg(test)]
#[rstest::rstest]
#[case(NamingScheme::None)]
#[case(NamingScheme::Short)]
#[case(NamingScheme::Full)]
fn test_plugin_config_from_naming_scheme(#[case] naming_scheme: NamingScheme) {
    let config = PluginConfig::from(naming_scheme);
    assert_eq!(config.naming_scheme, naming_scheme);
}
