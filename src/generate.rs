use std::{io, rc::Rc};

use iter_tools::Itertools;

use crate::model::StateConfig;

fn generate_type_definition(parent: Option<&str>, states: &StateConfig) -> String {
    let derives = parent
        .map(|parent_name| {
            [
                "#[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]",
                &format!(
                    "#[source({parent_name} = {parent_name}::{state_name})]",
                    state_name = states.name()
                ),
            ]
            .join("\n")
        })
        .unwrap_or("#[derive(States, Hash, Default, Debug, Clone, PartialEq, Eq)]".to_owned());
    let full_name = format!("{}{}", parent.unwrap_or_default(), states.name());
    match states {
        StateConfig::Single(_) => format!("{derives}\npub struct {full_name};"),
        StateConfig::Many(_, variants) => {
            let variants = variants.iter().map(|variant| variant.name()).join(", ");
            format!("{derives}\npub enum {full_name} {{ #[default] {variants} }}")
        }
    }
}

fn generate_all_type_definitions(states: &StateConfig) -> String {
    let own_type = generate_type_definition(None, states);
    match states {
        StateConfig::Single(_) => own_type,
        StateConfig::Many(_, variants) => {
            let variants = variants
                .iter()
                .map(|variant| generate_type_definition(Some(states.name()), variant))
                .join("\n");
            format!("{own_type}\n\n{variants}")
        }
    }
}

pub struct PluginConfig<'a> {
    pub plugin_name: &'a str,
    pub state_name: &'a str,
    pub states_module_name: &'a str,
}

impl Default for PluginConfig<'_> {
    fn default() -> Self {
        Self {
            plugin_name: "GeneratedStatesPlugin",
            state_name: "GameState",
            states_module_name: "states",
        }
    }
}

pub(crate) fn generate_states_plugin(states: Rc<StateConfig>, config: PluginConfig) -> String {
    let PluginConfig {
        plugin_name,
        state_name,
        states_module_name,
    } = config;
    let plugin = {
        let struct_decl = format!("pub struct {plugin_name};");
        let impl_block = format!("impl bevy::app::Plugin for {plugin_name}");
        let init = format!("app.init_state::<{states_module_name}::{state_name}>();");
        let build_fn = format!("fn build(&self, app: &mut bevy::app::App) {{ {init} }}");
        format!("{struct_decl}\n{impl_block} {{ {build_fn} }}")
    };
    let states_module = {
        format!(
            "pub mod {states_module_name} {{ use bevy::prelude::States; {type_definitions} }}",
            type_definitions = generate_all_type_definitions(&states),
        )
    };
    let source = states_module + &plugin;
    format_source(&source)
}

#[cfg(feature = "format")]
pub fn try_format_source(source: &str) -> io::Result<String> {
    duct::cmd!("rustfmt").stdin_bytes(source).read()
}

pub fn format_source(source: &str) -> String {
    #[cfg(feature = "format")]
    {
        try_format_source(source).unwrap_or_else(|_| source.to_owned())
    }
    #[cfg(not(feature = "format"))]
    {
        source.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use insta::assert_snapshot;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[timeout(Duration::from_millis(250))]
    async fn test_format_source() {
        let formatted = format_source("fn main(){println!(\"Hello, world!\");}");
        assert_snapshot!(formatted, @r#"
        fn main() {
            println!("Hello, world!");
        }
        "#);
    }

    #[rstest]
    fn test_generate_states_plugin() {
        let states = StateConfig::many("GameState", [
            StateConfig::single("Loading"),
            StateConfig::many("Ready", [
                StateConfig::many("Menu", [
                    StateConfig::single("Main"),
                    StateConfig::single("Options"),
                ]),
                StateConfig::many("Game", [
                    StateConfig::single("Playing"),
                    StateConfig::single("Paused"),
                    StateConfig::single("GameOver"),
                ]),
            ]),
        ]);
        assert_snapshot!(generate_states_plugin(Rc::new(states), Default::default()), @r"
        pub mod states {
            use bevy::prelude::States;
            #[derive(States, Hash, Default, Debug, Clone, PartialEq, Eq)]
            pub enum GameState {
                #[default]
                Loading,
                Ready,
            }

            #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
            #[source(GameState = GameState::Loading)]
            pub struct GameStateLoading;
            #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
            #[source(GameState = GameState::Ready)]
            pub enum GameStateReady {
                #[default]
                Menu,
                Game,
            }
        }
        pub struct GeneratedStatesPlugin;
        impl bevy::app::Plugin for GeneratedStatesPlugin {
            fn build(&self, app: &mut bevy::app::App) {
                app.init_state::<states::GameState>();
            }
        }
        ");
    }
}
