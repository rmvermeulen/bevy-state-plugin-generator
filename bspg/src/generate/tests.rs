use std::borrow::Cow;
use std::time::Duration;

use bevy_utils::default;
use insta::{assert_debug_snapshot, assert_snapshot};
use itertools::Itertools;
use rstest::{Context, rstest};
use speculoos::assert_that;
use speculoos::prelude::VecAssertions;

use crate::generate::core::{format_source, generate_debug_info};
use crate::generate::{GeneratorError, generate_state_plugin_source};
use crate::parsing::Node;
use crate::prelude::{NamingScheme, PluginConfig};
use crate::processing::{convert_nodes_into_plugin_source, process_nodes};
use crate::set_snapshot_suffix;
use crate::testing::node;

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
    let root_state = Node::enumeration(
        "GameState",
        [
            Node::singleton("Loading"),
            Node::enumeration(
                "Ready",
                [
                    Node::enumeration(
                        "Menu",
                        [Node::singleton("Main"), Node::singleton("Options")],
                    ),
                    Node::enumeration(
                        "Game",
                        [
                            Node::singleton("Playing"),
                            Node::singleton("Paused"),
                            Node::singleton("GameOver"),
                        ],
                    ),
                ],
            ),
        ],
    );
    let source = convert_nodes_into_plugin_source(vec![root_state], Default::default()).unwrap();
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
#[case("root.txt", "RootState", default())]
#[case("root.txt", "RootState", PluginConfig { root_state_name: None, ..default() })]
#[case("alpabet.txt", "A B C D E F G H I", default())]
#[case(
    "mixed-nested-states.txt",
    "A { B [C] } D { E F [ G H ] I }",
    default()
)]
fn test_generate_full_source(
    #[case] src_path: &str,
    #[case] source: &str,
    #[case] config: PluginConfig,
) {
    let root_state_name = config
        .root_state_name
        .clone()
        .unwrap_or(Cow::Borrowed("None"));
    set_snapshot_suffix!("{src_path}_{root_state_name}_{RUSTFMT}");
    assert_snapshot!(generate_state_plugin_source(source, config, Some(src_path)).unwrap());
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
    node: Vec<Node<'_>>,
    naming_scheme: NamingScheme,
    root_state_name: Option<String>,
) -> Result<Vec<String>, GeneratorError> {
    Ok(
        process_nodes(node, naming_scheme, root_state_name.clone().map(Cow::from))?
            .into_iter()
            .map(|node| format!("{} -> {}", node.name, node.resolved_name.unwrap()))
            .collect_vec(),
    )
}

#[rstest]
#[case::duplicates(node::duplicate_names())]
#[case::overlapping(node::overlapping_names())]
fn test_error_handling(
    #[context] context: Context,
    #[values(NamingScheme::Full, NamingScheme::Short, NamingScheme::None)]
    naming_scheme: NamingScheme,
    #[case] node: Node,
) {
    set_snapshot_suffix!("{}_{naming_scheme:?}", context.description.unwrap());
    assert_debug_snapshot!(generate_all_type_definitions(
        vec![node],
        naming_scheme,
        None
    ));
}

#[rstest]
#[case::enum_root_a(node::enum_root_a())]
#[case::enum_root_ab(node::enum_root_ab())]
#[case::enum_root_a_b(node::enum_root_a_b())]
#[case::enum_root_a_b_up_c(node::enum_root_a_b_up_c())]
#[case::list_root_a(node::list_root_a())]
#[case::list_root_ab(node::list_root_ab())]
#[case::list_root_a_b(node::list_root_a_b())]
#[case::list_root_a_b_up_c(node::list_root_a_b_up_c())]
#[case::nested_example(node::nested_example())]
fn snapshots(
    #[context] context: Context,
    #[values(NamingScheme::Full, NamingScheme::Short, NamingScheme::None)]
    naming_scheme: NamingScheme,
    #[case] node: Node,
) {
    set_snapshot_suffix!(
        "{}_{naming_scheme:?}{RUSTFMT}",
        context.description.unwrap()
    );
    assert_debug_snapshot!(generate_all_type_definitions(
        vec![node],
        naming_scheme,
        None
    ));
}
