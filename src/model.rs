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
    pub fn name(&self) -> &str {
        match self {
            NamingScheme::Full => "Full",
            NamingScheme::Short => "Short",
            NamingScheme::None => "None",
        }
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
