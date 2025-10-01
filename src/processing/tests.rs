use bevy_utils::default;

use crate::parsing::ParseNode;
use crate::processing::{NodeData, NodeType, flatten_parse_node};
use crate::testing::*;

#[rstest]
fn test_flatten_parse_node_singleton() {
    let results = flatten_parse_node(ParseNode::singleton("A"));
    assert_that!(results).is_equal_to(vec![NodeData {
        index: 0,
        parent: None,
        node_type: NodeType::Singleton,
        depth: 0,
        name: "A".to_string(),
        ..default()
    }]);
}

#[rstest]
#[case::enum_root_a(parse_node::enum_root_a())]
#[case::enum_root_ab(parse_node::enum_root_ab())]
#[case::enum_root_a_b(parse_node::enum_root_a_b())]
#[case::enum_root_a_b_up_c(parse_node::enum_root_a_b_up_c())]
fn test_flatten_parse_node_enums(#[context] context: Context, #[case] node: ParseNode) {
    set_snapshot_suffix!("{}", context.description.unwrap());
    assert_debug_snapshot!(flatten_parse_node(node));
}

#[rstest]
#[case::list_root_a(parse_node::list_root_a())]
#[case::list_root_ab(parse_node::list_root_ab())]
#[case::list_root_a_b(parse_node::list_root_a_b())]
#[case::list_root_a_b_up_c(parse_node::list_root_a_b_up_c())]
fn test_flatten_parse_node_lists(#[context] context: Context, #[case] node: ParseNode) {
    set_snapshot_suffix!("{}", context.description.unwrap());
    assert_debug_snapshot!(flatten_parse_node(node));
}
