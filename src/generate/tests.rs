use std::{convert::identity, time::Duration};

use super::*;
use insta::{assert_debug_snapshot, assert_snapshot};
use rstest::{fixture, rstest};

use crate::set_snapshot_suffix;
use crate::{NamingScheme, PluginConfig};

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
#[cfg_attr(
    feature = "comments",
    case("comments.txt", Rc::new(StateNode::enum_empty("GameState")),)
)]
#[case("simple.txt", Rc::new(StateNode::enumeration("GameState", [
    StateNode::singleton("Loading"),
    StateNode::enumeration("Ready", [
        StateNode::singleton("Menu"),
        StateNode::singleton("Game"),
    ]),
])))]
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
])))]
fn test_generate_plugin_source(#[case] src_path: &str, #[case] root_node: Rc<StateNode>) {
    set_snapshot_suffix!("{src_path}");
    assert_snapshot!(test_plugin_formatted(root_node, Default::default()));
}

#[cfg(feature = "lists")]
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
fn test_naming_scheme(#[case] src_path: &str, #[case] source: &str, #[case] scheme: NamingScheme) {
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
    let typenames = generate_all_type_definitions(&node, Context {
        source_state: Some(source.clone()),
        naming_scheme: NamingScheme::Full,
        ..Default::default()
    })
    .take()
    .into_iter()
    .map(|td| td.typename)
    .collect_vec();
    assert_debug_snapshot!(typenames, @r#"
    [
        "GameMenu",
        "GameMenuMain",
        "GameMenuOptions",
        "GameMenuOptionsGraphics",
        "GameMenuOptionsAudio",
        "GameMenuOptionsGameplay",
        "GameMenuContinue",
        "GameMenuContinueSave",
        "GameMenuContinueLoad",
    ]
    "#);
}

#[rstest]
fn test_generate_all_type_definitions_shortened(
    #[from(root_source_state)] source: SourceState,
    #[from(nested_node)] node: StateNode,
) {
    assert_debug_snapshot!(
        generate_all_type_definitions(&node, (source, NamingScheme::Short).into())
            .take().into_iter().map(|td| td.typename).collect_vec(),
        @r#"
    [
        "GameMenu",
        "MenuMain",
        "MenuOptions",
        "OptionsGraphics",
        "OptionsAudio",
        "OptionsGameplay",
        "MenuContinue",
        "ContinueSave",
        "ContinueLoad",
    ]
    "#);
}

#[rstest]
fn test_generate_all_type_definitions_none(
    #[from(root_source_state)] source: SourceState,
    #[from(nested_node)] node: StateNode,
) {
    assert_debug_snapshot!(
        generate_all_type_definitions(&node, (source, NamingScheme::None).into())
            .take().into_iter().map(|td| td.typename).collect_vec(),
        @r#"
    [
        "Menu",
        "Main",
        "Options",
        "Graphics",
        "Audio",
        "Gameplay",
        "Continue",
        "Save",
        "Load",
    ]
    "#);
}

#[rstest]
fn snapshots(
    #[values(NamingScheme::Full, NamingScheme::Short)] scheme: NamingScheme,
    #[from(root_source_state)] source: SourceState,
    #[from(nested_node)] node: StateNode,
) {
    set_snapshot_suffix!("{scheme:?}");
    assert_snapshot!(generate_all_type_definitions(
        &node,
        (source, scheme).into()
    ));
}

#[rstest]
fn snapshot1() {
    assert_snapshot!(generate_all_type_definitions(
        &StateNode::singleton("Alpha"),
        SourceState {
            name: "GameState".to_string(),
            variant: "Alpha".to_string()
        }.into()
    ), @r"
    #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
    #[source(GameState = GameState::Alpha)]
    pub struct Alpha;
    ");
}

#[rstest]
fn snapshot1a() {
    assert_snapshot!(generate_all_type_definitions( &StateNode::singleton("Alpha"), NamingScheme::Full.into()), @r"
        #[derive(bevy::prelude::States, Hash, Default, Debug, Clone, PartialEq, Eq)]
        pub struct Alpha;
        ");
}

#[rstest]
fn snapshot2() {
    assert_snapshot!(generate_all_type_definitions(
        &StateNode::enumeration("Alpha", [StateNode::singleton("Beta")]),
        (SourceState {
            name: "GameState".to_string(),
            variant: "Alpha".to_string()
        },
        NamingScheme::Full).into()
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
        &StateNode::enumeration("Alpha", [StateNode::singleton("Beta")]),
        NamingScheme::Full.into()
    ), @r"
        #[derive(bevy::prelude::States, Hash, Default, Debug, Clone, PartialEq, Eq)]
        pub enum Alpha { #[default] Beta }

        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(Alpha = Alpha::Beta)]
        pub struct AlphaBeta;
        ");
}

#[cfg(feature = "lists")]
#[rstest]
fn snapshot3() {
    assert_snapshot!(generate_all_type_definitions(
        &StateNode::list("Alpha", [StateNode::singleton("Beta")]),
        (SourceState::from(("GameState", "Alpha")), NamingScheme::Full).into()), @r"
        #[derive(bevy::prelude::SubStates, Hash, Default, Debug, Clone, PartialEq, Eq)]
        #[source(GameStateAlpha = GameStateAlpha::Beta)]
        pub struct GameStateAlphaBeta;
        ");
}

#[cfg(feature = "lists")]
#[rstest]
fn snapshot4() {
    assert_snapshot!(generate_all_type_definitions(
        &StateNode::list("List", [
            StateNode::singleton("Item1"),
            StateNode::enumeration("Item2", [
                StateNode::singleton("A"),
                StateNode::singleton("B"),
            ]),
            StateNode::singleton("Item3"),
        ]),
        (SourceState::from(("GameState", "Alpha")), NamingScheme::Full).into()), @r"
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
