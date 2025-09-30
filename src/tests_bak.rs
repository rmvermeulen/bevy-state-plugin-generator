// #[rstest]
// fn test_generate_all_type_definitions_full(#[from(state_node::ne)] node: StateNode) {
//     let typenames = generate_all_state_definitions(
//         node.into(),
//         Context {
//             parent_state: Some(source.clone()),
//             naming_scheme: NamingScheme::Full,
//             ..Default::default()
//         },
//     )
//     .inner()
//     .into_iter()
//     .map(|td| td.typename)
//     .collect_vec();
//     assert_debug_snapshot!(typenames, @r#"
//     [
//         "GameMenu",
//         "GameMenuMain",
//         "GameMenuOptions",
//         "GameMenuGameMenuOptionsGraphics",
//         "GameMenuGameMenuOptionsAudio",
//         "GameMenuGameMenuOptionsGameplay",
//         "GameMenuContinue",
//         "GameMenuGameMenuContinueSave",
//         "GameMenuGameMenuContinueLoad",
//     ]
//     "#);
// }

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
