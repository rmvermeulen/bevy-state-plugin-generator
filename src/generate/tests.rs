use std::time::Duration;

use insta::{assert_debug_snapshot, assert_snapshot};
use itertools::Itertools;
use rstest::{Context, rstest};

use crate::generate::{format_source, generate_debug_info, generate_state_plugin_source};
use crate::parsing::ParseNode;
use crate::processing::{apply_naming_scheme, flatten_parse_node, try_parse_node_into_final_source};
use crate::testing::parse_node;
use crate::{NamingScheme, PluginConfig, set_snapshot_suffix};

#[cfg(feature = "rustfmt")]
const RUSTFMT: &str = "_rustfmt";
#[cfg(not(feature = "rustfmt"))]
const RUSTFMT: &str = "_no_rustfmt";

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
    assert_snapshot!(try_parse_node_into_final_source(root_state, Default::default()).unwrap());
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

fn generate_all_type_definitions(node: ParseNode<'_>, naming_scheme: NamingScheme) -> Vec<String> {
    let mut nodes = flatten_parse_node(node);
    apply_naming_scheme(naming_scheme, &mut nodes).unwrap();
    nodes
        .into_iter()
        .map(|node| format!("{} -> {}", node.name, node.resolved_name.unwrap()))
        .collect_vec()
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
