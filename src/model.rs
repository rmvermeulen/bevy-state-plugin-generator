#[derive(PartialEq, Debug, Clone)]
pub struct SourceState {
    pub name: String,
    pub variant: String,
}

impl SourceState {
    pub fn display_name(&self) -> String {
        self.name.clone()
    }
    pub fn display_variant(&self) -> String {
        format!("{}::{}", self.name, self.variant)
    }
}
/// How state-names are determined
#[derive(Default, Debug, Clone, Copy)]
pub enum NamingScheme {
    /// Name includes the names of all ancestors
    #[default]
    Full,
    // TODO: implement this
    /// Name includes only the name of the immediate parent
    Shortened,
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
