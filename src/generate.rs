use std::{io, rc::Rc};

use iter_tools::Itertools;

use crate::model::StateConfig;
use crate::parse_config;

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
        StateConfig::Single(_) | StateConfig::List(_, _) => {
            format!("{derives}\npub struct {full_name};")
        }
        StateConfig::Enum(_, variants) => {
            let variants = variants.iter().map(|variant| variant.name()).join(", ");
            format!("{derives}\npub enum {full_name} {{ #[default] {variants} }}")
        }
    }
}

fn generate_all_type_definitions(parent: Option<&str>, states: &StateConfig) -> String {
    let own_type = generate_type_definition(parent, states);
    match states {
        StateConfig::Single(_) => own_type,
        StateConfig::Enum(_, variants) | StateConfig::List(_, variants) => {
            let parent = Some(states.name());
            let variants = variants
                .iter()
                .map(|variant| generate_all_type_definitions(parent, variant))
                .join("\n");
            format!("{own_type}\n\n{variants}")
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PluginConfig<'a> {
    pub plugin_name: &'a str,
    pub state_name: &'a str,
    pub states_module_name: &'a str,
}

impl<'a> PluginConfig<'a> {
    pub fn new(
        plugin_name: &'a str,
        state_name: &'a str,
        states_module_name: &'a str,
    ) -> PluginConfig<'a> {
        PluginConfig {
            plugin_name,
            state_name,
            states_module_name,
        }
    }
}

pub fn generate_debug_info(src_path: &str, source: &str, states: &Vec<Rc<StateConfig>>) -> String {
    format!(
        "// src: {src_path}\n{lines}\n// {states:?}",
        lines = source.lines().map(|line| format!("// {line}")).join("\n")
    )
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
    let header = r#"
        #![allow(missing_docs)]
        use bevy::prelude::AppExtStates;
    "#;
    let plugin = {
        let struct_decl = format!("pub struct {plugin_name};");
        let impl_block = format!("impl bevy::app::Plugin for {plugin_name}");
        let init = format!("app.init_state::<{states_module_name}::{state_name}>();");
        let build_fn = format!("fn build(&self, app: &mut bevy::app::App) {{ {init} }}");
        format!("{struct_decl}\n{impl_block} {{ {build_fn} }}")
    };
    let states_module = format!(
        "pub mod {states_module_name} {{ use bevy::prelude::States; {type_definitions} }}",
        type_definitions = generate_all_type_definitions(None, &states),
    );
    let source = [header, &states_module, &plugin].join("\n");
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

pub fn generate_full_source<P: AsRef<str> + std::fmt::Display, S: AsRef<str>>(
    src_path: P,
    source: S,
    plugin_config: PluginConfig,
) -> Result<String, String> {
    let source = source.as_ref();
    let (_, state_config) = parse_config(source).map_err(|e| format!("{e:?}"))?;
    let states = state_config
        .into_iter()
        .map(|item| Rc::new(item.into()))
        .collect_vec();

    let debug_info = generate_debug_info(src_path.as_ref(), &source, &states);
    let plugin_source = match generate_plugin_source(states, plugin_config) {
        Ok(plugin_source) => plugin_source,
        Err(message) => message,
    };
    Ok([debug_info, plugin_source].join("\n"))
}

pub fn generate_plugin_source(
    states: Vec<Rc<StateConfig>>,
    plugin_config: PluginConfig,
) -> Result<String, String> {
    let state_config = Rc::new(if states.is_empty() {
        StateConfig::single(plugin_config.state_name.to_string())
    } else {
        StateConfig::variants(plugin_config.state_name.to_string(), states)
    });

    Ok(generate_states_plugin(state_config, plugin_config))
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use insta::assert_snapshot;
    use rstest::rstest;

    macro_rules! set_snapshot_suffix {
        ($($expr:expr),*) => {
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_suffix(format!($($expr,)*));
            let _guard = settings.bind_to_scope();
        }
    }

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
        let states = StateConfig::variants("GameState", [
            StateConfig::single("Loading"),
            StateConfig::variants("Ready", [
                StateConfig::variants("Menu", [
                    StateConfig::single("Main"),
                    StateConfig::single("Options"),
                ]),
                StateConfig::variants("Game", [
                    StateConfig::single("Playing"),
                    StateConfig::single("Paused"),
                    StateConfig::single("GameOver"),
                ]),
            ]),
        ]);
        assert_snapshot!(generate_states_plugin(Rc::new(states), Default::default()), @r"
        #![allow(missing_docs)]
        use bevy::prelude::AppExtStates;

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

            #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
            #[source(Ready = Ready::Menu)]
            pub enum ReadyMenu {
                #[default]
                Main,
                Options,
            }

            #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
            #[source(Menu = Menu::Main)]
            pub struct MenuMain;
            #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
            #[source(Menu = Menu::Options)]
            pub struct MenuOptions;
            #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
            #[source(Ready = Ready::Game)]
            pub enum ReadyGame {
                #[default]
                Playing,
                Paused,
                GameOver,
            }

            #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
            #[source(Game = Game::Playing)]
            pub struct GamePlaying;
            #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
            #[source(Game = Game::Paused)]
            pub struct GamePaused;
            #[derive(SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
            #[source(Game = Game::GameOver)]
            pub struct GameGameOver;
        }
        pub struct GeneratedStatesPlugin;
        impl bevy::app::Plugin for GeneratedStatesPlugin {
            fn build(&self, app: &mut bevy::app::App) {
                app.init_state::<states::GameState>();
            }
        }
        ");
    }

    #[rstest]
    #[case("root.txt", "RootState")]
    #[case("fruits.txt", "Apple Orange { O1 O2 }")]
    fn test_generate_debug_info(#[case] src_path: &str, #[case] source: &str) {
        set_snapshot_suffix!("{src_path}");
        assert_snapshot!(generate_debug_info(src_path, source, &vec![]));
    }

    #[rstest]
    #[case("fruits.txt", vec![
            Rc::new(StateConfig::single("Loading")),
            Rc::new(StateConfig::variants("Ready", [
                StateConfig::variants("Menu", [
                    StateConfig::single("Main"),
                    StateConfig::single("Options"),
                ]),
                StateConfig::variants("Game", [
                    StateConfig::single("Playing"),
                    StateConfig::single("Paused"),
                    StateConfig::single("GameOver"),
                ]),
            ])),
        ], PluginConfig::default())]
    fn test_generate_plugin_source(
        #[case] src_path: &str,
        #[case] source: Vec<Rc<StateConfig>>,
        #[case] plugin_config: PluginConfig,
    ) {
        set_snapshot_suffix!("{src_path}");
        assert_snapshot!(match generate_plugin_source(source, plugin_config) {
            Ok(plugin_source) => plugin_source,
            Err(message) => message,
        });
    }

    #[rstest]
    #[case("root.txt", "RootState", PluginConfig::default())]
    #[case(
        "mixed-nested-states.txt",
        "A { B [C[ } D { E F [ G H ] I }",
        PluginConfig::default()
    )]
    fn test_generate_full_source(
        #[case] src_path: &str,
        #[case] source: &str,
        #[case] plugin_config: PluginConfig,
    ) {
        set_snapshot_suffix!("{src_path}");
        assert_snapshot!(
            match generate_full_source(src_path, source, plugin_config) {
                Ok(full_source) => full_source,
                Err(message) => message,
            }
        );
    }
}
