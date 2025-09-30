use std::time::Duration;

use insta::{assert_debug_snapshot, assert_snapshot};
use itertools::Itertools;
use rstest::rstest;

use crate::generate::{format_source, generate_debug_info, generate_state_plugin_source};
use crate::parsing::ParseNode;
use crate::processing::{NodeData, apply_naming_scheme, flatten_parse_node,
                        parse_node_into_final_source};
use crate::testing::{node_data, parse_node};
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
    let root_state = ParseNode::enumeration(
        "GameState",
        [
            ParseNode::singleton("Loading"),
            ParseNode::enumeration(
                "Ready",
                [
                    ParseNode::enumeration(
                        "Menu",
                        [
                            ParseNode::singleton("Main"),
                            ParseNode::singleton("Options"),
                        ],
                    ),
                    ParseNode::enumeration(
                        "Game",
                        [
                            ParseNode::singleton("Playing"),
                            ParseNode::singleton("Paused"),
                            ParseNode::singleton("GameOver"),
                        ],
                    ),
                ],
            ),
        ],
    );
    assert_snapshot!(parse_node_into_final_source(root_state, Default::default()).unwrap());
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

#[rstest]
#[case("single_node", node_data::single_node())]
#[case("nested_example", node_data::nested_example())]
fn test_apply_naming_scheme(
    #[case] id: &str,
    #[case] mut nodes: Vec<NodeData>,
    #[values(NamingScheme::Short, NamingScheme::Full, NamingScheme::None)]
    naming_scheme: NamingScheme,
) {
    set_snapshot_suffix!("{id}_{naming_scheme}");
    apply_naming_scheme(naming_scheme, &mut nodes).unwrap();
    assert_debug_snapshot!(
        nodes
            .into_iter()
            .map(|node| format!("{} -> {}", node.name, node.resolved_name.unwrap()))
            .collect_vec()
    );
}

fn generate_all_type_definitions(node: ParseNode<'_>, naming_scheme: NamingScheme) -> Vec<String> {
    let mut nodes = flatten_parse_node(node);
    apply_naming_scheme(naming_scheme, &mut nodes).unwrap();
    nodes
        .into_iter()
        .map(|node| format!("{} -> {}", node.name, node.resolved_name.unwrap()))
        .collect_vec()
}

#[rstest]
fn test_generate_all_type_definitions_full(
    #[from(parse_node::nested_example)] node: ParseNode<'_>,
) {
    let typenames = generate_all_type_definitions(node, NamingScheme::Full);
    assert_debug_snapshot!(typenames, @r#"
    [
        "Menu -> Menu",
        "Main -> MenuMain",
        "Options -> MenuOptions",
        "Game -> MenuGame",
        "Graphics -> MenuOptionsGraphics",
        "Audio -> MenuOptionsAudio",
        "Gameplay -> MenuOptionsGameplay",
        "Save -> MenuGameSave",
        "Load -> MenuGameLoad",
    ]
    "#);
}

#[rstest]
fn test_generate_all_type_definitions_shortened(
    #[from(parse_node::nested_example)] node: ParseNode,
) {
    assert_debug_snapshot!(
        generate_all_type_definitions(node, NamingScheme::Short),
        @r#"
    [
        "Menu -> Menu",
        "Main -> MenuMain",
        "Options -> MenuOptions",
        "Game -> MenuGame",
        "Graphics -> OptionsGraphics",
        "Audio -> OptionsAudio",
        "Gameplay -> OptionsGameplay",
        "Save -> GameSave",
        "Load -> GameLoad",
    ]
    "#);
}
