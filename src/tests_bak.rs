use insta::assert_debug_snapshot;
use itertools::Itertools;
use rstest::rstest;

use crate::NamingScheme;
use crate::parsing::ParseNode;
use crate::processing::{NodeData, apply_naming_scheme, flatten_parse_node};
use crate::testing::parse_node;

fn generate_all_type_definitions(
    node: ParseNode<'_>,
    naming_scheme: NamingScheme,
) -> Vec<NodeData> {
    let mut nodes = flatten_parse_node(node);
    apply_naming_scheme(naming_scheme, &mut nodes).unwrap();
    nodes
}

#[rstest]
fn test_generate_all_type_definitions_full(
    #[from(parse_node::nested_example)] node: &ParseNode<'_>,
) {
    let typenames = generate_all_type_definitions(node.clone(), NamingScheme::Full)
        .into_iter()
        .map(|node| (node.name, node.resolved_name.unwrap()))
        .collect_vec();
    assert_debug_snapshot!(typenames, @r#"
    [
        (
            "Menu",
            "Menu",
        ),
        (
            "Main",
            "MenuMain",
        ),
        (
            "Options",
            "MenuOptions",
        ),
        (
            "Continue",
            "MenuContinue",
        ),
        (
            "Graphics",
            "OptionsMenuGraphics",
        ),
        (
            "Audio",
            "OptionsMenuAudio",
        ),
        (
            "Gameplay",
            "OptionsMenuGameplay",
        ),
        (
            "Save",
            "ContinueMenuSave",
        ),
        (
            "Load",
            "ContinueMenuLoad",
        ),
    ]
    "#);
}

// TODO:
// #[rstest]
// fn test_generate_all_type_definitions_shortened(
//     #[from(root_parent_state)] source: ParentState,
//     #[from(nested_node)] node: StateNode,
// ) {
//     assert_debug_snapshot!(
//         generate_all_state_definitions(node.into(), (source, NamingScheme::Short).into())
//             .inner().into_iter().map(|td| td.typename).collect_vec(),
//         @r#"
//     [
//         "GameMenu",
//         "MenuMain",
//         "MenuOptions",
//         "OptionsGraphics",
//         "OptionsAudio",
//         "OptionsGameplay",
//         "MenuContinue",
//         "ContinueSave",
//         "ContinueLoad",
//     ]
//     "#);
// }

// TODO:
// #[rstest]
// fn test_generate_all_type_definitions_none(
//     #[from(root_parent_state)] source: ParentState,
//     #[from(nested_node)] node: StateNode,
// ) {
//     assert_debug_snapshot!(
//         generate_all_state_definitions(node.into(), (source, NamingScheme::None).into())
//             .inner().into_iter().map(|td| td.typename).collect_vec(),
//         @r#"
//     [
//         "Menu",
//         "Main",
//         "Options",
//         "Graphics",
//         "Audio",
//         "Gameplay",
//         "Continue",
//         "Save",
//         "Load",
//     ]
//     "#);
// }

// TODO:
// #[rstest]
// fn snapshots(
//     #[values(NamingScheme::Full, NamingScheme::Short)] naming_scheme: NamingScheme,
//     #[from(root_parent_state)] source: ParentState,
//     #[from(nested_node)] node: StateNode,
// ) {
//     let suffix = cfg!(feature = "rustfmt")
//         .then_some("_rustfmt")
//         .unwrap_or_default();
//     set_snapshot_suffix!("{naming_scheme:?}{suffix}");
//     assert_snapshot!(generate_all_state_definitions(
//         node.into(),
//         (source, naming_scheme).into()
//     ));
// }

// TODO:
// #[rstest]
// fn snapshot1() {
//     let suffix = cfg!(feature = "rustfmt")
//         .then_some("_rustfmt")
//         .unwrap_or_default();
//     set_snapshot_suffix!("snapshot1{suffix}");
//     assert_snapshot!(generate_all_state_definitions(
//         StateNode::singleton("Alpha").into(),
//         ParentState::new("GameState", "Alpha", None).into()
//     ));
// }

// TODO:
// #[rstest]
// fn snapshot1a() {
//     let suffix = cfg!(feature = "rustfmt")
//         .then_some("_rustfmt")
//         .unwrap_or_default();
//     set_snapshot_suffix!("snapshot1a{suffix}");
//     assert_snapshot!(generate_all_state_definitions(
//         StateNode::singleton("Alpha").into(),
//         NamingScheme::Full.into()
//     ));
// }

// TODO:
// #[rstest]
// fn snapshot2() {
//     let suffix = cfg!(feature = "rustfmt")
//         .then_some("_rustfmt")
//         .unwrap_or_default();
//     set_snapshot_suffix!("snapshot2{suffix}");
//     assert_snapshot!(generate_all_state_definitions(
//         StateNode::enumeration("Alpha", [StateNode::singleton("Beta")]).into(),
//         (
//             ParentState::new("GameState", "Alpha", None),
//             NamingScheme::Full
//         )
//             .into()
//     ));
// }

// TODO:
// #[rstest]
// fn snapshot2a() {
//     let suffix = cfg!(feature = "rustfmt")
//         .then_some("_rustfmt")
//         .unwrap_or_default();
//     set_snapshot_suffix!("snapshot2a{suffix}");
//     assert_snapshot!(generate_all_state_definitions(
//         StateNode::enumeration("Alpha", [StateNode::singleton("Beta")]).into(),
//         NamingScheme::Full.into()
//     ));
// }

// TODO:
// #[rstest]
// fn snapshot3() {
//     let suffix = cfg!(feature = "rustfmt")
//         .then_some("_rustfmt")
//         .unwrap_or_default();
//     set_snapshot_suffix!("snapshot3{suffix}");
//     assert_snapshot!(generate_all_state_definitions(
//         StateNode::list("Alpha", [StateNode::singleton("Beta")]).into(),
//         (
//             ParentState::new("GameState", "Alpha", None),
//             NamingScheme::Full
//         )
//             .into()
//     ));
// }

// TODO:
// #[rstest]
// fn snapshot4() {
//     let suffix = cfg!(feature = "rustfmt")
//         .then_some("_rustfmt")
//         .unwrap_or_default();
//     set_snapshot_suffix!("snapshot4{suffix}");
//     assert_snapshot!(generate_all_state_definitions(
//         StateNode::list(
//             "List",
//             [
//                 StateNode::singleton("Item1"),
//                 StateNode::enumeration(
//                     "Item2",
//                     [StateNode::singleton("A"), StateNode::singleton("B"),]
//                 ),
//                 StateNode::singleton("Item3"),
//             ]
//         )
//         .into(),
//         (
//             ParentState::new("GameState", "Alpha", None),
//             NamingScheme::Full
//         )
//             .into()
//     ));
// }
