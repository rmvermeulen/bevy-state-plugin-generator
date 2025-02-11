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

#[derive(Default, Debug, Clone, Copy)]
pub enum NamingScheme {
    #[default]
    Full,
    // TODO: implement this
    Shortened,
}

#[derive(Debug, Clone, Copy)]
pub struct PluginConfig<'a> {
    pub plugin_name: &'a str,
    pub state_name: &'a str,
    pub states_module_name: &'a str,
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
