use std::{io, rc::Rc};

use iter_tools::Itertools;

use crate::model::{SourceState, StateNode, StateTree};
use crate::parser::parse_states_file;
use crate::{NamingScheme, PluginConfig};

fn generate_all_type_definitions(
    source_state: Option<SourceState>,
    root_node: &StateNode,
    scheme: NamingScheme,
) -> String {
    const DERIVES: &str = "Hash, Default, Debug, Clone, PartialEq, Eq";
    let typename = if scheme == NamingScheme::None {
        root_node.name().to_string()
    } else {
        source_state
            .clone()
            .map(|source_state| format!("{}{}", source_state.name(), root_node.name()))
            .unwrap_or_else(|| root_node.name().to_string())
    };
    let root_typedef = {
        let derives = source_state
            .clone()
            .map(|source_state| {
                let source = source_state.name();
                let variant = source_state.name_and_variant();
                [
                    format!("#[derive(bevy::prelude::SubStates, {DERIVES})]"),
                    format!("#[source({source} = {variant})]"),
                ]
                .join("\n")
            })
            .unwrap_or(format!("#[derive(bevy::prelude::States, {DERIVES})]"));

        match root_node {
            StateNode::Singleton(_) | StateNode::List(_, _) => {
                format!("{derives}\npub struct {typename};")
            }
            StateNode::Enum(_, variants) => {
                let variants = variants.iter().map(|variant| variant.name()).join(", ");
                format!("{derives}\npub enum {typename} {{ #[default] {variants} }}")
            }
        }
    };
    match root_node {
        StateNode::Singleton(_) => root_typedef,
        StateNode::Enum(_, variants) => {
            let root_name = root_node.name().to_string();
            let variants = variants
                .iter()
                .map(|child_node| {
                    generate_all_type_definitions(
                        Some(SourceState {
                            name: match scheme {
                                NamingScheme::Short | NamingScheme::None => root_name.clone(),
                                NamingScheme::Full => typename.clone(),
                            },
                            variant: child_node.name().to_string(),
                        }),
                        child_node,
                        scheme,
                    )
                })
                .join("\n");
            format!("{root_typedef}\n\n{variants}")
        }
        StateNode::List(_, variants) => variants
            .iter()
            .map(|child_node| {
                generate_all_type_definitions(
                    Some(SourceState {
                        name: typename.clone(),
                        variant: child_node.name().to_string(),
                    }),
                    child_node,
                    scheme,
                )
            })
            .join("\n"),
    }
}

pub fn generate_debug_info(src_path: &str, source: &str) -> String {
    format!(
        "// src: {src_path}\n{lines}",
        lines = source.lines().map(|line| format!("// {line}")).join("\n")
    )
}

pub(crate) fn generate_plugin_source(root_state: Rc<StateNode>, config: PluginConfig) -> String {
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
        type_definitions = generate_all_type_definitions(None, &root_state, config.scheme),
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

pub fn generate_state_plugin_source<P: AsRef<str> + std::fmt::Display, S: AsRef<str>>(
    src_path: P,
    source: S,
    plugin_config: PluginConfig,
) -> Result<String, String> {
    let source = source.as_ref();
    let parse_tree =
        parse_states_file(source, plugin_config.state_name).map_err(|e| e.to_string())?;
    let parse_tree_size = parse_tree.get_size();

    let root_node: Rc<StateNode> = Rc::new(parse_tree.into());
    let state_tree_size = root_node.get_size();

    assert_eq!(parse_tree_size, state_tree_size);

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
    use rstest::{fixture, rstest};
    use speculoos::prelude::*;

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
        let states = StateNode::enumeration("GameState", [
            StateNode::singleton("Loading"),
            StateNode::enumeration("Ready", [
                StateNode::enumeration("Menu", [
                    StateNode::singleton("Main"),
                    StateNode::singleton("Options"),
                ]),
                StateNode::enumeration("Game", [
                    StateNode::singleton("Playing"),
                    StateNode::singleton("Paused"),
                    StateNode::singleton("GameOver"),
                ]),
            ]),
        ]);
        assert_snapshot!(generate_plugin_source(Rc::new(states), Default::default()));
    }

    #[rstest]
    #[case("root.txt", "RootState")]
    #[case("fruits.txt", "Apple Orange { O1 O2 }")]
    fn test_generate_debug_info(#[case] src_path: &str, #[case] source: &str) {
        set_snapshot_suffix!("{src_path}");
        assert_snapshot!(generate_debug_info(src_path, source));
    }

    fn test_plugin_formatted(root_node: Rc<StateNode>, plugin_config: PluginConfig) -> String {
        format_source(generate_plugin_source(root_node, plugin_config))
    }

    #[rstest]
    #[case("fruits.txt", Rc::new(StateNode::enumeration("GameState", [
            StateNode::singleton("Loading"),
            StateNode::enumeration("Ready", [
                StateNode::enumeration("Menu", [
                    StateNode::singleton("Main"),
                    StateNode::singleton("Options"),
                ]),
                StateNode::enumeration("Game", [
                    StateNode::singleton("Playing"),
                    StateNode::singleton("Paused"),
                    StateNode::singleton("GameOver"),
                ]),
            ]),
        ])), PluginConfig::default())]
    fn test_generate_plugin_source(
        #[case] src_path: &str,
        #[case] root_node: Rc<StateNode>,
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
            generate_state_plugin_source(src_path, source, plugin_config).unwrap_or_else(identity)
        );
    }

    #[rstest]
    #[case("root.txt", "RootState", NamingScheme::Full)]
    #[case("root.txt", "RootState", NamingScheme::Short)]
    fn test_naming_scheme(
        #[case] src_path: &str,
        #[case] source: &str,
        #[case] scheme: NamingScheme,
    ) {
        set_snapshot_suffix!("{src_path}_{scheme:?}");
        assert_snapshot!(
            generate_state_plugin_source(src_path, source, PluginConfig {
                scheme,
                ..Default::default()
            })
            .unwrap_or_else(identity)
        );
    }

    #[fixture]
    fn root_source_state() -> SourceState {
        SourceState {
            name: "Game".to_string(),
            variant: "Menu".to_string(),
        }
    }
    #[fixture]
    fn nested_node() -> StateNode {
        StateNode::enumeration("Menu", [
            StateNode::singleton("Main"),
            StateNode::enumeration("Options", [
                StateNode::singleton("Graphics"),
                StateNode::singleton("Audio"),
                StateNode::singleton("Gameplay"),
            ]),
            StateNode::enumeration("Continue", [
                StateNode::singleton("Save"),
                StateNode::singleton("Load"),
            ]),
        ])
    }

    #[rstest]
    fn test_generate_all_type_definitions_full(
        #[from(root_source_state)] source: SourceState,
        #[from(nested_node)] node: StateNode,
    ) {
        let typedef_result =
            generate_all_type_definitions(Some(source.clone()), &node, NamingScheme::Full);
        assert_that!(typedef_result).contains(" GameMenu");
        assert_that!(typedef_result).contains(" GameMenuMain");
        assert_that!(typedef_result).contains(" GameMenuOptions");
        assert_that!(typedef_result).contains(" GameMenuOptionsGraphics");
    }

    #[rstest]
    fn test_generate_all_type_definitions_shortened(
        #[from(root_source_state)] source: SourceState,
        #[from(nested_node)] node: StateNode,
    ) {
        let typedef_result =
            generate_all_type_definitions(Some(source), &node, NamingScheme::Short);
        assert_that!(typedef_result).contains(" GameMenu");
        assert_that!(typedef_result).contains(" MenuMain");
        assert_that!(typedef_result).contains(" MenuOptions");
        assert_that!(typedef_result).contains(" OptionsGraphics");
    }

    #[rstest]
    fn test_generate_all_type_definitions_none(
        #[from(root_source_state)] source: SourceState,
        #[from(nested_node)] node: StateNode,
    ) {
        let typedef_result = generate_all_type_definitions(Some(source), &node, NamingScheme::None);
        assert_that!(typedef_result).contains(" Menu");
        assert_that!(typedef_result).contains(" Main");
        assert_that!(typedef_result).contains(" Options");
        assert_that!(typedef_result).contains(" Graphics");
    }

    #[rstest]
    fn snapshots(
        #[values(NamingScheme::Full, NamingScheme::Short)] scheme: NamingScheme,
        #[from(root_source_state)] source: SourceState,
        #[from(nested_node)] node: StateNode,
    ) {
        set_snapshot_suffix!("{scheme:?}");
        assert_snapshot!(generate_all_type_definitions(Some(source), &node, scheme));
    }

    #[rstest]
    fn snapshot1() {
        assert_snapshot!(generate_all_type_definitions(
            Some(SourceState {
                name: "GameState".to_string(),
                variant: "Alpha".to_string()
            }),
            &StateNode::singleton("Alpha"),
            NamingScheme::Full
        ), @r"
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameState = GameState::Alpha)]
        pub struct GameStateAlpha;
        ");
    }

    #[rstest]
    fn snapshot1a() {
        assert_snapshot!(generate_all_type_definitions(None, &StateNode::singleton("Alpha"), NamingScheme::Full), @r"
        #[derive(bevy::prelude::States, Hash, Default, Debug, Clone, PartialEq, Eq)]
        pub struct Alpha;
        ");
    }

    #[rstest]
    fn snapshot2() {
        assert_snapshot!(generate_all_type_definitions(
            Some(SourceState {
                name: "GameState".to_string(),
                variant: "Alpha".to_string()
            }),
            &StateNode::enumeration("Alpha", [StateNode::singleton("Beta")]),
            NamingScheme::Full
        ), @r"
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameState = GameState::Alpha)]
        pub enum GameStateAlpha { #[default] Beta }

        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameStateAlpha = GameStateAlpha::Beta)]
        pub struct GameStateAlphaBeta;
        ");
    }

    #[rstest]
    fn snapshot2a() {
        assert_snapshot!(generate_all_type_definitions(
            None, &StateNode::enumeration("Alpha", [StateNode::singleton("Beta")]),
            NamingScheme::Full
        ), @r"
        #[derive(bevy::prelude::States, Hash, Default, Debug, Clone, PartialEq, Eq)]
        pub enum Alpha { #[default] Beta }

        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(Alpha = Alpha::Beta)]
        pub struct AlphaBeta;
        ");
    }

    #[rstest]
    fn snapshot3() {
        assert_snapshot!(generate_all_type_definitions(
            Some(SourceState {
                name: "GameState".to_string(),
                variant: "Alpha".to_string()
            }),
            &StateNode::list("Alpha", [StateNode::singleton("Beta")]),
            NamingScheme::Full
        ), @r"
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameStateAlpha = GameStateAlpha::Beta)]
        pub struct GameStateAlphaBeta;
        ");
    }

    #[rstest]
    fn snapshot4() {
        assert_snapshot!(generate_all_type_definitions(
            Some(SourceState {
                name: "GameState".to_string(),
                variant: "Alpha".to_string()
            }),
            &StateNode::list("List", [
                StateNode::singleton("Item1"),
                StateNode::enumeration("Item2", [
                    StateNode::singleton("A"),
                    StateNode::singleton("B"),
                ]),
                StateNode::singleton("Item3"),
            ]),
            NamingScheme::Full
        ), @r"
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameStateList = GameStateList::Item1)]
        pub struct GameStateListItem1;
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameStateList = GameStateList::Item2)]
        pub enum GameStateListItem2 { #[default] A, B }

        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameStateListItem2 = GameStateListItem2::A)]
        pub struct GameStateListItem2A;
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameStateListItem2 = GameStateListItem2::B)]
        pub struct GameStateListItem2B;
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameStateList = GameStateList::Item3)]
        pub struct GameStateListItem3;
        ");
    }
}
