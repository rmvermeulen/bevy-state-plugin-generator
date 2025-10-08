use nom::Parser;
use nom::branch::alt;
use speculoos::prelude::*;

use crate::parsing::parsers::*;
use crate::parsing::*;
use crate::testing::*;

#[rstest]
#[case("Name", "Name")]
#[case("Name ", "Name")]
#[case("Name123", "Name123")]
#[case("Name_123", "Name_123")]
fn test_identifier(#[case] input: &str, #[case] token: &str) {
    assert_that!(identifier(input))
        .is_ok()
        .map(|(_, token)| token)
        .is_equal_to(Identifier::from(token));
}

#[rstest]
#[case("//\nHello\n", "")]
#[case("//Hello\n", "Hello")]
#[case("// Hello\n", "Hello")]
#[case("// Hello// \n", "Hello//")]
#[case("// Hello, how are you? \n", "Hello, how are you?")]
fn test_comment(#[case] input: &str, #[case] expected: &str) {
    assert_that!(comment(input))
        .is_ok()
        .map(|(_, token)| token)
        .is_equal_to(Comment::from(expected));
}

#[rstest]
#[case("//Hello\n", "Hello")]
#[case("// Hello\n", "Hello")]
#[case("// Hello// \n", "Hello//")]
#[case("// Hello, how are you? \n", "Hello, how are you?")]
fn test_parse_comment(#[case] input: &str, #[case] expected: &str) {
    assert_that!(parse_comment(input))
        .is_ok()
        .map(|(_, token)| token)
        .is_equal_to(Node::comment(expected));
}

#[rstest]
#[case("{", Token::OpenEnum)]
#[case("}", Token::CloseEnum)]
#[case("[", Token::OpenList)]
#[case("]", Token::CloseList)]
fn test_single_char_tokens(#[case] input: &str, #[case] expected: Token) {
    let mut parser = alt((open_enum, close_enum, open_list, close_list));

    assert_that!(parser.parse(input))
        .is_ok()
        .is_equal_to(("", expected));
}

#[rstest]
#[case("Root,", ",", Node::singleton("Root"))]
#[case("  Root,", ",", Node::singleton("Root"))]
#[case("First, Second", ", Second", Node::singleton("First"))]
fn test_parse_singleton(#[case] input: &str, #[case] rest: &str, #[case] node: Node) {
    assert_that!(parse_singleton(input).unwrap()).is_equal_to((rest, node));
}

#[rstest]
fn test_parse_enum_empty() {
    assert_compact_debug_snapshot!(parse_enum("Root{}").unwrap(), @r#"("", Enum(Identifier("Root"), []))"#);
    assert_compact_debug_snapshot!(parse_enum("Root {}").unwrap(), @r#"("", Enum(Identifier("Root"), []))"#);
}

#[rstest]
#[case("Root {A}", node::enum_root_a())]
#[case("Root { A}", node::enum_root_a())]
#[case("Root {A }", node::enum_root_a())]
#[case("Root { A }", node::enum_root_a())]
fn test_parse_enum_single(#[case] input: &str, #[case] node: Node) {
    assert_that!(parse_enum(input).unwrap()).is_equal_to(("", node));
}

#[rstest]
#[case("Root {A,B}", node::enum_root_ab())]
#[case("Root { A B }", node::enum_root_ab())]
#[case("Root { A { B } }", node::enum_root_a_b())]
#[case("Root { A { B } C }", node::enum_root_a_b_up_c())]
#[case("Root { A { B }, C }", node::enum_root_a_b_up_c())]
fn test_parse_enum_variants(#[case] input: &str, #[case] node: Node) {
    assert_that!(parse_enum(input).unwrap()).is_equal_to(("", node));
}

#[rstest]
#[case::just_a_comma("Root {,}", Node::enumeration("Root", [ ]))]
#[case::mora_commas("Root {,,,,}", Node::enumeration("Root", [ ]))]
#[case::comma_after_variant("Root {A,}", node::enum_root_a())]
#[case::comma_before_variant("Root {,A}", node::enum_root_a())]
#[case::comma_between_variants("Root {A,B}", node::enum_root_ab())]
fn test_parse_enum_optional_commas(#[case] input: &str, #[case] node: Node) {
    assert_that!(parse_enum(input).unwrap()).is_equal_to(("", node));
}

#[rstest]
#[case("Root []", Node::list_empty("Root"))]
#[case("Root[]", Node::list_empty("Root"))]
#[case("Root[A]", node::list_root_a())]
#[case("Root[A,B]", node::list_root_ab())]
fn test_parse_list(#[case] input: &str, #[case] node: Node) {
    assert_that!(parse_list(input).unwrap()).is_equal_to(("", node));
}

#[rstest]
#[case("Root")]
#[case("Root {A}")]
fn test_parse_node(#[case] input: &str) {
    set_snapshot_suffix!("{}", input.replace(" ", "_"));
    assert_compact_debug_snapshot!(parse_node(input));
}

#[rstest]
fn test_parse_node_nested_enums() {
    let input = "Root { A { B, C {D E {F G}} H } I J }";
    assert_debug_snapshot!(parse_node(input));
}

#[rstest]
#[case("//Comment", Node::comment("Comment"))]
#[case("// Comment", Node::comment("Comment"))]
fn test_parse_node_with_comments(#[case] input: &str, #[case] comment: Node) {
    assert_that!(parse_node(input)).is_ok_containing(("", comment));
}

#[rstest]
fn test_parse_node_messy_example() {
    let input = "Root [ A { B, C [D E {F G}] H } I J ]";
    assert_debug_snapshot!(parse_node(input));
}

#[rstest]
fn test_parse_list_incomplete() {
    assert_compact_debug_snapshot!(parse_list("Root [ A"), @r#"Err(Error(Error { input: "", code: Tag }))"#);
}

#[rstest]
fn test_parse_enum_incomplete() {
    assert_compact_debug_snapshot!(parse_enum("Root { A"), @r#"Err(Error(Error { input: "", code: Tag }))"#);
}

#[rstest]
#[case("A", vec![Node::singleton("A")])]
#[case("A B", vec![Node::singleton("A"), Node::singleton("B")] )]
#[case("A,B", vec![Node::singleton("A"), Node::singleton("B")] )]
#[case("A, B", vec![Node::singleton("A"), Node::singleton("B")] )]
#[case("A{B}", vec![Node::enumeration("A", [ Node::singleton("B") ])] )]
#[case("A{B,C D,}", vec![Node::enumeration("A", [
    Node::singleton("B"),
    Node::singleton("C"),
    Node::singleton("D")
])])]
#[case("A{B{C{D{E{F}}}}}", vec![Node::enumeration("A", [
    Node::enumeration("B", [
        Node::enumeration("C", [
            Node::enumeration("D", [
                Node::enumeration("E", [
                    Node::singleton("F")
                ])
            ])
        ])
    ])
])])]
#[ case("//A//{ B C }", vec![ Node::comment("A//{ B C }") ])]
#[ case("A//{ B C }", vec![
    Node::singleton("A"),
    Node::comment("{ B C }")
])]
fn test_parse_config(#[case] input: &str, #[case] expected: Vec<Node>) {
    assert_that!(parse_config(input))
        .named(&format!("\"{input}\""))
        .is_ok()
        .is_equal_to(("", expected));
}

#[rstest]
#[case("A//\n{ B C }", vec![
    Node::singleton("A"),
    Node::comment("")
], "{ B C }")]
#[case("A {\n // B\n C\n}...", vec![ Node::enumeration("A", [
    Node::comment("B"),
    Node::singleton("C")
]) ], "...")]
fn test_parse_config_incomplete(
    #[case] input: &str,
    #[case] expected: Vec<Node>,
    #[case] rest: &str,
) {
    assert_that!(parse_config(input))
        .named(&format!("\"{input}\""))
        .is_ok()
        .is_equal_to((rest, expected));
}

#[rstest]
#[case("Main", Node::singleton("Main"))]
#[case("Main{}", Node::Enum("Main".into(), Default::default()))]
#[case("Main{A}", Node::enumeration("Main", [Node::singleton("A")]))]
#[case("Main{A,B}", Node::enumeration("Main", [
    Node::singleton("A"),
    Node::singleton("B"),
]))]
fn test_parse_node_try_from_str(#[case] input: &str, #[case] expected: Node) {
    assert_that!(Node::try_from(input)).is_ok_containing(expected);
}

#[rstest]
#[case("RootState", 1)]
#[case("Root, Root2", 2)]
#[case("A B C D E F G H I", 9)]
#[case("A { B [C] } D { E F [ G H ] I }", 2)]
fn test_parse_state_text(#[case] source: &str, #[case] root_count: usize) {
    let (_, parse_nodes) = parse_config(source).unwrap();
    assert_that!(parse_nodes).has_length(root_count);
}
