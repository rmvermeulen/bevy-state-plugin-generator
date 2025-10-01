use std::time::Duration;

use bevy_utils::default;
use insta::{assert_debug_snapshot, assert_snapshot};
use itertools::Itertools;
use rstest::{Context, rstest};
use speculoos::assert_that;
use speculoos::prelude::VecAssertions;

use crate::generate::{format_source, generate_debug_info, generate_state_plugin_source};
use crate::parsing::ParseNode;
use crate::processing::{apply_naming_scheme, flatten_parse_node, try_parse_node_into_final_source};
use crate::testing::parse_node;
use crate::{GeneratorError, NamingScheme, PluginConfig, set_snapshot_suffix};

#[cfg(feature = "rustfmt")]
const RUSTFMT: &str = "_rustfmt";
#[cfg(not(feature = "rustfmt"))]
const RUSTFMT: &str = "_no_rustfmt";

#[rstest]
#[timeout(Duration::from_millis(250))]
#[async_std::test]
async fn test_format_source() {
    set_snapshot_suffix!("formatted{RUSTFMT}");
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
    let source = try_parse_node_into_final_source(root_state, Default::default()).unwrap();
    assert_that!(source.matches(" mod ").collect_vec()).has_length(1);
    assert_snapshot!(source);
}

#[rstest]
#[case("root.txt", "RootState")]
#[case("fruits.txt", "Apple Orange { O1 O2 }")]
fn test_generate_debug_info(#[case] src_path: &str, #[case] source: &str) {
    set_snapshot_suffix!("{src_path}{RUSTFMT}");
    assert_snapshot!(generate_debug_info(src_path, source));
}

#[rstest]
#[case("RootState", 1)]
#[case("A B C D E F G H I", 9)]
#[case("A { B [C] } D { E F [ G H ] I }", 2)]
fn test_parse_state_text(#[case] source: &str, #[case] root_count: usize) {
    use crate::parsing::parse_states_text;
    let parse_nodes = parse_states_text(source).unwrap();
    assert_that!(parse_nodes).has_length(root_count);
}

#[rstest]
#[case("root.txt", "RootState")]
#[case("alpabet.txt", "A B C D E F G H I")]
#[case("mixed-nested-states.txt", "A { B [C] } D { E F [ G H ] I }")]
fn test_generate_full_source(#[case] src_path: &str, #[case] source: &str) {
    set_snapshot_suffix!("{src_path}{RUSTFMT}");
    assert_snapshot!(generate_state_plugin_source(source, default(), Some(src_path)).unwrap());
}

#[rstest]
#[case("root.txt", "RootState", PluginConfig { root_state_name: None, ..default() })]
#[case(
    "mixed-nested-states.txt",
    "A { B [C] } D { E F [ G H ] I }",
    PluginConfig { root_state_name: None, ..default() }
)]
fn test_generate_full_source_no_implicit_root(
    #[case] src_path: &str,
    #[case] source: &str,
    #[case] plugin_config: PluginConfig,
) {
    set_snapshot_suffix!("{src_path}{RUSTFMT}");
    assert_snapshot!(generate_state_plugin_source(source, plugin_config, Some(src_path)).unwrap());
}

#[rstest]
#[case(Some("root.txt"))]
#[case(None)]
fn test_naming_scheme(
    #[case] src_path: Option<&str>,
    #[values(NamingScheme::Full, NamingScheme::Short, NamingScheme::None)]
    naming_scheme: NamingScheme,
) {
    let src_path_display = src_path.unwrap_or("no_src");
    set_snapshot_suffix!("{src_path_display}_{naming_scheme}{RUSTFMT}");
    let result = generate_state_plugin_source(
        "RootState",
        PluginConfig {
            naming_scheme,
            ..Default::default()
        },
        src_path,
    )
    .unwrap();

    assert_that!(result.matches(" mod ").collect_vec()).has_length(1);
    assert_snapshot!(result);
}

fn generate_all_type_definitions(
    node: ParseNode<'_>,
    naming_scheme: NamingScheme,
) -> Result<Vec<String>, GeneratorError> {
    let mut nodes = flatten_parse_node(node);
    apply_naming_scheme(naming_scheme, &mut nodes)?;
    Ok(nodes
        .into_iter()
        .map(|node| format!("{} -> {}", node.name, node.resolved_name.unwrap()))
        .collect_vec())
}

#[rstest]
#[case::duplicates(parse_node::duplicate_names())]
#[case::overlapping(parse_node::overlapping_names())]
fn test_error_handling(
    #[context] context: Context,
    #[values(NamingScheme::Full, NamingScheme::Short, NamingScheme::None)]
    naming_scheme: NamingScheme,
    #[case] node: ParseNode,
) {
    set_snapshot_suffix!("{}_{naming_scheme:?}", context.description.unwrap());
    assert_debug_snapshot!(generate_all_type_definitions(node, naming_scheme));
}

#[rstest]
#[case::enum_root_a(parse_node::enum_root_a())]
#[case::enum_root_ab(parse_node::enum_root_ab())]
#[case::enum_root_a_b(parse_node::enum_root_a_b())]
#[case::enum_root_a_b_up_c(parse_node::enum_root_a_b_up_c())]
#[case::list_root_a(parse_node::list_root_a())]
#[case::list_root_ab(parse_node::list_root_ab())]
#[case::list_root_a_b(parse_node::list_root_a_b())]
#[case::list_root_a_b_up_c(parse_node::list_root_a_b_up_c())]
#[case::nested_example(parse_node::nested_example())]
fn snapshots(
    #[context] context: Context,
    #[values(NamingScheme::Full, NamingScheme::Short, NamingScheme::None)]
    naming_scheme: NamingScheme,
    #[case] node: ParseNode,
) {
    set_snapshot_suffix!(
        "{}_{naming_scheme:?}{RUSTFMT}",
        context.description.unwrap()
    );
    assert_debug_snapshot!(generate_all_type_definitions(node, naming_scheme));
}
