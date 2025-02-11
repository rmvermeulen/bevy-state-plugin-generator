use std::{io, rc::Rc};

use iter_tools::Itertools;

use crate::PluginConfig;
use crate::model::SourceState;
use crate::nodes::Node;
use crate::parser::parse_states_file;

fn generate_type_definition(source_state: Option<SourceState>, node: &Node) -> String {
    let (typename, derives) = source_state
        .map(|source_state| {
            ( format!("{}{}", source_state.display_name(), node.name()),
            [
                "#[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]",
                &format!(
                    "#[source({source} = {variant})]",
                    source = source_state.display_name(),
                    variant = source_state.display_variant()
                ),
            ]
            .join("\n"))
        })
        .unwrap_or((
            node.name().to_string(),
            "#[derive(bevy::prelude::States, Hash, Default, Debug, Clone, PartialEq, Eq)]"
                .to_owned(),
        ));

    match node {
        Node::Singleton(_) | Node::List(_, _) => {
            format!("{derives}\npub struct {typename};")
        }
        Node::Enum(_, variants) => {
            let variants = variants.iter().map(|variant| variant.name()).join(", ");
            format!("{derives}\npub enum {typename} {{ #[default] {variants} }}")
        }
    }
}

fn generate_all_type_definitions(parent: Option<SourceState>, root_node: &Node) -> String {
    let own_type = generate_type_definition(parent, root_node);
    match root_node {
        Node::Singleton(_) => own_type,
        Node::Enum(_, variants) | Node::List(_, variants) => {
            let variants = variants
                .iter()
                .map(|variant| {
                    generate_all_type_definitions(
                        Some(SourceState {
                            name: root_node.name().to_string().clone(),
                            variant: variant.name().to_string(),
                        }),
                        variant,
                    )
                })
                .join("\n");
            format!("{own_type}\n\n{variants}")
        }
    }
}

pub fn generate_debug_info(src_path: &str, source: &str) -> String {
    format!(
        "// src: {src_path}\n{lines}",
        lines = source.lines().map(|line| format!(" // {line}")).join("\n")
    )
}

pub(crate) fn generate_plugin_source(root_state: Rc<Node>, config: PluginConfig) -> String {
    let PluginConfig {
        plugin_name,
        state_name,
        states_module_name,
        scheme: _,
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
        "pub mod {states_module_name} {{ use bevy::prelude::StateSet; {type_definitions} }}",
        type_definitions = generate_all_type_definitions(None, &root_state),
    );
    [header, &states_module, &plugin].join("\n")
}

#[cfg(feature = "format")]
pub fn try_format_source(source: &str) -> io::Result<String> {
    duct::cmd!("rustfmt")
        .stdin_bytes(source)
        .stderr_to_stdout()
        .read()
}

pub fn format_source<S: AsRef<str>>(source: S) -> String {
    let source = source.as_ref();
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
    let root_node =
        parse_states_file(source, plugin_config.state_name).map_err(|e| format!("{e:?}"))?;

    let debug_info = generate_debug_info(src_path.as_ref(), source);
    let plugin_source = generate_plugin_source(root_node, plugin_config);
    let source = [debug_info, plugin_source].join("\n");
    Ok(format_source(source))
}

#[cfg(test)]
mod tests {
    use std::{convert::identity, time::Duration};

    use super::*;
    use insta::assert_snapshot;
    use rstest::rstest;

    use crate::{NamingScheme, PluginConfig};

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
        let states = Node::enumeration("GameState", [
            Node::singleton("Loading"),
            Node::enumeration("Ready", [
                Node::enumeration("Menu", [
                    Node::singleton("Main"),
                    Node::singleton("Options"),
                ]),
                Node::enumeration("Game", [
                    Node::singleton("Playing"),
                    Node::singleton("Paused"),
                    Node::singleton("GameOver"),
                ]),
            ]),
        ]);
        assert_snapshot!(generate_plugin_source(Rc::new(states), Default::default()), @r"
                #![allow(missing_docs)]
                use bevy::prelude::AppExtStates;
            
        pub mod states { use bevy::prelude::StateSet; #[derive(bevy::prelude::States, Hash, Default, Debug, Clone, PartialEq, Eq)]
        pub enum GameState { #[default] Loading, Ready }

        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameState = GameState::Loading)]
        pub struct GameStateLoading;
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameState = GameState::Ready)]
        pub enum GameStateReady { #[default] Menu, Game }

        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(Ready = Ready::Menu)]
        pub enum ReadyMenu { #[default] Main, Options }

        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(Menu = Menu::Main)]
        pub struct MenuMain;
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(Menu = Menu::Options)]
        pub struct MenuOptions;
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(Ready = Ready::Game)]
        pub enum ReadyGame { #[default] Playing, Paused, GameOver }

        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(Game = Game::Playing)]
        pub struct GamePlaying;
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(Game = Game::Paused)]
        pub struct GamePaused;
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(Game = Game::GameOver)]
        pub struct GameGameOver; }
        pub struct GeneratedStatesPlugin;
        impl bevy::app::Plugin for GeneratedStatesPlugin { fn build(&self, app: &mut bevy::app::App) { app.init_state::<states::GameState>(); } }
        ");
    }

    #[rstest]
    #[case("root.txt", "RootState")]
    #[case("fruits.txt", "Apple Orange { O1 O2 }")]
    fn test_generate_debug_info(#[case] src_path: &str, #[case] source: &str) {
        set_snapshot_suffix!("{src_path}");
        assert_snapshot!(generate_debug_info(src_path, source));
    }

    fn test_plugin_formatted(root_node: Rc<Node>, plugin_config: PluginConfig) -> String {
        format_source(generate_plugin_source(root_node, plugin_config))
    }

    #[rstest]
    #[case("fruits.txt", Rc::new(Node::enumeration("GameState", [
            Node::singleton("Loading"),
            Node::enumeration("Ready", [
                Node::enumeration("Menu", [
                    Node::singleton("Main"),
                    Node::singleton("Options"),
                ]),
                Node::enumeration("Game", [
                    Node::singleton("Playing"),
                    Node::singleton("Paused"),
                    Node::singleton("GameOver"),
                ]),
            ]),
        ])), PluginConfig::default())]
    fn test_generate_plugin_source(
        #[case] src_path: &str,
        #[case] root_node: Rc<Node>,
        #[case] plugin_config: PluginConfig,
    ) {
        set_snapshot_suffix!("{src_path}");
        assert_snapshot!(test_plugin_formatted(root_node, plugin_config));
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
            generate_full_source(src_path, source, plugin_config).unwrap_or_else(identity)
        );
    }

    #[rstest]
    #[case("root.txt", "RootState", NamingScheme::Full)]
    #[case("root.txt", "RootState", NamingScheme::Shortened)]
    fn test_naming_scheme(#[case] src: &str, #[case] source: &str, #[case] scheme: NamingScheme) {
        let plugin_config = PluginConfig::new("GeneratedStatesPlugin", src, "states", scheme);
        set_snapshot_suffix!("{src}_{scheme:?}");
        assert_snapshot!(generate_full_source(src, source, plugin_config).unwrap_or_else(identity));
    }
}
