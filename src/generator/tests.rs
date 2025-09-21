use std::convert::identity;
use std::time::Duration;

use insta::{assert_debug_snapshot, assert_snapshot};
use rstest::{fixture, rstest};

use super::*;
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
    let states = StateNode::enumeration(
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
    assert_snapshot!(generate_plugin_source(Rc::new(states), Default::default()));
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

fn test_plugin_formatted(root_node: Rc<StateNode>, plugin_config: PluginConfig) -> String {
    format_source(generate_plugin_source(root_node, plugin_config))
}

#[rstest]
#[case("comments.txt", Rc::new(StateNode::enum_empty("GameState")))]
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
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("{src_path}{suffix}");
    assert_snapshot!(test_plugin_formatted(root_node, Default::default()));
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
        generate_state_plugin_source(src_path, source, plugin_config).unwrap_or_else(identity)
    );
}

#[rstest]
#[case("root.txt", "RootState", NamingScheme::Full)]
#[case("root.txt", "RootState", NamingScheme::Short)]
fn test_naming_scheme(#[case] src_path: &str, #[case] source: &str, #[case] scheme: NamingScheme) {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("{src_path}{suffix}");
    assert_snapshot!(
        generate_state_plugin_source(
            src_path,
            source,
            PluginConfig {
                scheme,
                ..Default::default()
            }
        )
        .unwrap_or_else(identity)
    );
}

#[fixture]
fn root_parent_state() -> ParentState {
    ParentState::new("Game", "Menu")
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
    let typenames = generate_all_type_definitions(
        &node,
        Context {
            parent_state: Some(source.clone()),
            naming_scheme: NamingScheme::Full,
            ..Default::default()
        },
    )
    .take()
    .into_iter()
    .map(|td| td.typename)
    .collect_vec();
    assert_debug_snapshot!(typenames, @r#"
    [
        "GameMenuState",
        "GameMenuMainState",
        "GameMenuOptionsState",
        "GameMenuOptionsGraphicsState",
        "GameMenuOptionsAudioState",
        "GameMenuOptionsGameplayState",
        "GameMenuContinueState",
        "GameMenuContinueSaveState",
        "GameMenuContinueLoadState",
    ]
    "#);
}

#[rstest]
fn test_generate_all_type_definitions_shortened(
    #[from(root_parent_state)] source: ParentState,
    #[from(nested_node)] node: StateNode,
) {
    assert_debug_snapshot!(
        generate_all_type_definitions(&node, (source, NamingScheme::Short).into())
            .take().into_iter().map(|td| td.typename).collect_vec(),
        @r#"
    [
        "GameMenuState",
        "MenuMainState",
        "MenuOptionsState",
        "OptionsGraphicsState",
        "OptionsAudioState",
        "OptionsGameplayState",
        "MenuContinueState",
        "ContinueSaveState",
        "ContinueLoadState",
    ]
    "#);
}

#[rstest]
fn test_generate_all_type_definitions_none(
    #[from(root_parent_state)] source: ParentState,
    #[from(nested_node)] node: StateNode,
) {
    assert_debug_snapshot!(
        generate_all_type_definitions(&node, (source, NamingScheme::None).into())
            .take().into_iter().map(|td| td.typename).collect_vec(),
        @r#"
    [
        "MenuState",
        "MainState",
        "OptionsState",
        "GraphicsState",
        "AudioState",
        "GameplayState",
        "ContinueState",
        "SaveState",
        "LoadState",
    ]
    "#);
}

#[rstest]
fn snapshots(
    #[values(NamingScheme::Full, NamingScheme::Short)] scheme: NamingScheme,
    #[from(root_parent_state)] source: ParentState,
    #[from(nested_node)] node: StateNode,
) {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("{scheme:?}{suffix}");
    assert_snapshot!(generate_all_type_definitions(
        &node,
        (source, scheme).into()
    ));
}

#[rstest]
fn snapshot1() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("snapshot1{suffix}");
    assert_snapshot!(generate_all_type_definitions(
        &StateNode::singleton("Alpha"),
        ParentState::new("GameState", "Alpha").into()
    ));
}

#[rstest]
fn snapshot1a() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("snapshot1a{suffix}");
    assert_snapshot!(generate_all_type_definitions(
        &StateNode::singleton("Alpha"),
        NamingScheme::Full.into()
    ));
}

#[rstest]
fn snapshot2() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("snapshot2{suffix}");
    assert_snapshot!(generate_all_type_definitions(
        &StateNode::enumeration("Alpha", [StateNode::singleton("Beta")]),
        (ParentState::new("GameState", "Alpha"), NamingScheme::Full).into()
    ));
}

#[rstest]
fn snapshot2a() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("snapshot2a{suffix}");
    assert_snapshot!(generate_all_type_definitions(
        &StateNode::enumeration("Alpha", [StateNode::singleton("Beta")]),
        NamingScheme::Full.into()
    ));
}

#[rstest]
fn snapshot3() {
    let suffix = cfg!(feature = "rustfmt")
        .then_some("_rustfmt")
        .unwrap_or_default();
    set_snapshot_suffix!("snapshot3{suffix}");
    assert_snapshot!(generate_all_type_definitions(
        &StateNode::list("Alpha", [StateNode::singleton("Beta")]),
        (
            ParentState::from(("GameState", "Alpha")),
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
    assert_snapshot!(generate_all_type_definitions(
        &StateNode::list(
            "List",
            [
                StateNode::singleton("Item1"),
                StateNode::enumeration(
                    "Item2",
                    [StateNode::singleton("A"), StateNode::singleton("B"),]
                ),
                StateNode::singleton("Item3"),
            ]
        ),
        (
            ParentState::from(("GameState", "Alpha")),
            NamingScheme::Full
        )
            .into()
    ));
}
