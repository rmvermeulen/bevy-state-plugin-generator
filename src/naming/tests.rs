use crate::naming::{NodeData, NodeType, flatten_parse_node};
use crate::parsing::ParseNode;
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
    }]);
}

#[rstest]
#[case(parse_node::enum_root_a(), "enum_root_a")]
#[case(parse_node::enum_root_ab(), "enum_root_ab")]
#[case(parse_node::enum_root_a_b(), "enum_root_a_b")]
#[case(parse_node::enum_root_a_b_up_c(), "enum_root_a_b_up_c")]
fn test_flatten_parse_node_enums(#[case] node: ParseNode, #[case] name: &str) {
    set_snapshot_suffix!("{name}");
    assert_debug_snapshot!(flatten_parse_node(node));
}

#[rstest]
#[case(parse_node::list_root_a(), "list_root_a")]
#[case(parse_node::list_root_ab(), "list_root_ab")]
#[case(parse_node::list_root_a_b(), "list_root_a_b")]
#[case(parse_node::list_root_a_b_up_c(), "list_root_a_b_up_c")]
fn test_flatten_parse_node_lists(#[case] node: ParseNode, #[case] name: &str) {
    set_snapshot_suffix!("{name}");
    assert_debug_snapshot!(flatten_parse_node(node));
}
