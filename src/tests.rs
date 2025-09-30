use std::time::Duration;

use insta::assert_snapshot;
use rstest::rstest;

use crate::generate::{format_source, generate_debug_info, generate_state_plugin_source};
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

// #[rstest]
// fn test_generate_states_plugin() {
//     let root_state = StateNode::enumeration(
//         "GameState",
//         [
//             StateNode::singleton("Loading"),
//             StateNode::enumeration(
//                 "Ready",
//                 [
//                     StateNode::enumeration(
//                         "Menu",
//                         [
//                             StateNode::singleton("Main"),
//                             StateNode::singleton("Options"),
//                         ],
//                     ),
//                     StateNode::enumeration(
//                         "Game",
//                         [
//                             StateNode::singleton("Playing"),
//                             StateNode::singleton("Paused"),
//                             StateNode::singleton("GameOver"),
//                         ],
//                     ),
//                 ],
//             ),
//         ],
//     );
//     assert_snapshot!(
//         generate_state_plugin_source(root_state.into(), Default::default(), None).unwrap()
//     );
// }

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

// TODO: fix test
// #[rstest]
// #[case("comments.txt", StateNode::enum_empty("GameState"))]
// #[case("simple.txt", StateNode::enumeration("GameState", [
//     StateNode::singleton("Loading"),
//     StateNode::enumeration("Ready", [
//         StateNode::singleton("Menu"),
//         StateNode::singleton("Game"),
//     ]),
// ]))]
// #[case("fruits.txt", StateNode::enumeration("GameState", [
//     StateNode::singleton("Loading"),
//     StateNode::enumeration("Ready", [
//         StateNode::enumeration("Menu", [
//             StateNode::singleton("Main"),
//             StateNode::singleton("Options"),
//         ]),
//         StateNode::enumeration("Game", [
//             StateNode::singleton("Playing"),
//             StateNode::singleton("Paused"),
//             StateNode::singleton("GameOver"),
//         ]),
//     ]),
// ]))]
// fn test_generate_plugin_source_inner(#[case] src_path: &str, #[case] root_node: StateNode) {
//     let suffix = cfg!(feature = "rustfmt")
//         .then_some("_rustfmt")
//         .unwrap_or_default();
//     set_snapshot_suffix!("{src_path}{suffix}");
// assert_snapshot!(format_source(generate_state_plugin_source(
//     root_node.into(),
//     Default::default(),
//     None
// )));
// }

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
    assert_snapshot!(generate_state_plugin_source(source, plugin_config, Some(src_path)).unwrap());
}

#[rstest]
#[case("root.txt", "RootState", NamingScheme::Full)]
#[case("root.txt", "RootState", NamingScheme::Short)]
fn test_naming_scheme(
    #[case] src_path: &str,
    #[case] source: &str,
    #[case] naming_scheme: NamingScheme,
) {
    use crate::generate::generate_state_plugin_source;

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
        .unwrap()
    );
}
