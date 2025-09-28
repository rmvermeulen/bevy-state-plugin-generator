use std::convert::identity;
use std::time::Duration;

use insta::{assert_debug_snapshot, assert_snapshot};
use itertools::Itertools;
use rstest::{fixture, rstest};

use crate::generator::context::Context;
use crate::generator::generate::{format_source, generate_debug_info,
                                 generate_plugin_source_from_defined_states};
use crate::generator::generate_state_plugin_source;
use crate::generator::state_defs::generate_all_state_definitions;
use crate::models::{ParentState, StateNode};
use crate::{NamingScheme, PluginConfig, set_snapshot_suffix};

#[rstest]
#[timeout(Duration::from_millis(250))]
#[async_std::test]
async fn test_format_source() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("formatted{suffix}");
    let formatted = format_source("fn main(){println!(\"Hello, world!\");}");
    assert_snapshot!(formatted);
}

#[rstest]
fn test_generate_states_plugin() {
    let root_state = StateNode::enumeration(
        "GameState",
        [
            StateNode::singleton("Loading"),
            StateNode::enumeration(
                "Ready",
                [
                    StateNode::enumeration(
                        "Menu",
                        [
                            StateNode::singleton("Main"),
                            StateNode::singleton("Options"),
                        ],
                    ),
                    StateNode::enumeration(
                        "Game",
                        [
                            StateNode::singleton("Playing"),
                            StateNode::singleton("Paused"),
                            StateNode::singleton("GameOver"),
                        ],
                    ),
                ],
            ),
        ],
    );
    assert_snapshot!(generate_plugin_source_from_defined_states(
        root_state.into(),
        Default::default()
    ));
}

#[rstest]
#[case("root.txt", "RootState")]
#[case("fruits.txt", "Apple Orange { O1 O2 }")]
fn test_generate_debug_info(#[case] src_path: &str, #[case] source: &str) {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("{src_path}{suffix}");
    assert_snapshot!(generate_debug_info(src_path, source));
}

#[rstest]
#[case("comments.txt", StateNode::enum_empty("GameState"))]
#[case("simple.txt", StateNode::enumeration("GameState", [
    StateNode::singleton("Loading"),
    StateNode::enumeration("Ready", [
        StateNode::singleton("Menu"),
        StateNode::singleton("Game"),
    ]),
]))]
#[case("fruits.txt", StateNode::enumeration("GameState", [
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
]))]
fn test_generate_plugin_source_inner(#[case] src_path: &str, #[case] root_node: StateNode) {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("{src_path}{suffix}");
    assert_snapshot!(format_source(generate_plugin_source_from_defined_states(
        root_node.into(),
        Default::default()
    )));
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
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("{src_path}{suffix}");
    assert_snapshot!(
        generate_state_plugin_source(source, plugin_config, Some(src_path))
            .unwrap_or_else(identity)
    );
}

#[rstest]
#[case("root.txt", "RootState", NamingScheme::Full)]
#[case("root.txt", "RootState", NamingScheme::Short)]
fn test_naming_scheme(
    #[case] src_path: &str,
    #[case] source: &str,
    #[case] naming_scheme: NamingScheme,
) {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("{src_path}_{naming_scheme}{suffix}");
    assert_snapshot!(
        generate_state_plugin_source(
            source,
            PluginConfig {
                naming_scheme,
                ..Default::default()
            },
            Some(src_path),
        )
        .unwrap_or_else(identity)
    );
}

#[fixture]
fn root_parent_state() -> ParentState {
    ParentState::new("Game", "Menu", None)
}

#[fixture]
fn nested_node() -> StateNode {
    StateNode::enumeration(
        "Menu",
        [
            StateNode::singleton("Main"),
            StateNode::enumeration(
                "Options",
                [
                    StateNode::singleton("Graphics"),
                    StateNode::singleton("Audio"),
                    StateNode::singleton("Gameplay"),
                ],
            ),
            StateNode::enumeration(
                "Continue",
                [StateNode::singleton("Save"), StateNode::singleton("Load")],
            ),
        ],
    )
}

#[rstest]
fn test_generate_all_type_definitions_full(
    #[from(root_parent_state)] source: ParentState,
    #[from(nested_node)] node: StateNode,
) {
    let typenames = generate_all_state_definitions(
        node.into(),
        Context {
            parent_state: Some(source.clone()),
            naming_scheme: NamingScheme::Full,
            ..Default::default()
        },
    )
    .inner()
    .into_iter()
    .map(|td| td.typename)
    .collect_vec();
    assert_debug_snapshot!(typenames, @r#"
    [
        "GameMenu",
        "GameMenuMain",
        "GameMenuOptions",
        "GameMenuGameMenuOptionsGraphics",
        "GameMenuGameMenuOptionsAudio",
        "GameMenuGameMenuOptionsGameplay",
        "GameMenuContinue",
        "GameMenuGameMenuContinueSave",
        "GameMenuGameMenuContinueLoad",
    ]
    "#);
}

#[rstest]
fn test_generate_all_type_definitions_shortened(
    #[from(root_parent_state)] source: ParentState,
    #[from(nested_node)] node: StateNode,
) {
    assert_debug_snapshot!(
        generate_all_state_definitions(node.into(), (source, NamingScheme::Short).into())
            .inner().into_iter().map(|td| td.typename).collect_vec(),
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
    #[from(root_parent_state)] source: ParentState,
    #[from(nested_node)] node: StateNode,
) {
    assert_debug_snapshot!(
        generate_all_state_definitions(node.into(), (source, NamingScheme::None).into())
            .inner().into_iter().map(|td| td.typename).collect_vec(),
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
    #[values(NamingScheme::Full, NamingScheme::Short)] naming_scheme: NamingScheme,
    #[from(root_parent_state)] source: ParentState,
    #[from(nested_node)] node: StateNode,
) {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("{naming_scheme:?}{suffix}");
    assert_snapshot!(generate_all_state_definitions(
        node.into(),
        (source, naming_scheme).into()
    ));
}

#[rstest]
fn snapshot1() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("snapshot1{suffix}");
    assert_snapshot!(generate_all_state_definitions(
        StateNode::singleton("Alpha").into(),
        ParentState::new("GameState", "Alpha", None).into()
    ));
}

#[rstest]
fn snapshot1a() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("snapshot1a{suffix}");
    assert_snapshot!(generate_all_state_definitions(
        StateNode::singleton("Alpha").into(),
        NamingScheme::Full.into()
    ));
}

#[rstest]
fn snapshot2() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("snapshot2{suffix}");
    assert_snapshot!(generate_all_state_definitions(
        StateNode::enumeration("Alpha", [StateNode::singleton("Beta")]).into(),
        (
            ParentState::new("GameState", "Alpha", None),
            NamingScheme::Full
        )
            .into()
    ));
}

#[rstest]
fn snapshot2a() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("snapshot2a{suffix}");
    assert_snapshot!(generate_all_state_definitions(
        StateNode::enumeration("Alpha", [StateNode::singleton("Beta")]).into(),
        NamingScheme::Full.into()
    ));
}

#[rstest]
fn snapshot3() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("snapshot3{suffix}");
    assert_snapshot!(generate_all_state_definitions(
        StateNode::list("Alpha", [StateNode::singleton("Beta")]).into(),
        (
            ParentState::new("GameState", "Alpha", None),
            NamingScheme::Full
        )
            .into()
    ));
}

#[rstest]
fn snapshot4() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("snapshot4{suffix}");
    assert_snapshot!(generate_all_state_definitions(
        StateNode::list(
            "List",
            [
                StateNode::singleton("Item1"),
                StateNode::enumeration(
                    "Item2",
                    [StateNode::singleton("A"), StateNode::singleton("B"),]
                ),
                StateNode::singleton("Item3"),
            ]
        )
        .into(),
        (
            ParentState::new("GameState", "Alpha", None),
            NamingScheme::Full
        )
            .into()
    ));
}
