use nom::Parser;
use nom::branch::alt;

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
        .is_equal_to(ParseNode::comment(expected));
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
#[case("Root,", ",", ParseNode::singleton("Root"))]
#[case("  Root,", ",", ParseNode::singleton("Root"))]
#[case("First, Second", ", Second", ParseNode::singleton("First"))]
fn test_parse_singleton(#[case] input: &str, #[case] rest: &str, #[case] node: ParseNode) {
    assert_that!(parse_singleton(input).unwrap()).is_equal_to((rest, node));
}

#[rstest]
fn test_parse_enum_empty() {
    assert_compact_debug_snapshot!(parse_enum("Root{}").unwrap(), @r#"("", Enum(Identifier("Root"), []))"#);
    assert_compact_debug_snapshot!(parse_enum("Root {}").unwrap(), @r#"("", Enum(Identifier("Root"), []))"#);
}

#[rstest]
#[case("Root {A}", parse_node::enum_root_a())]
#[case("Root { A}", parse_node::enum_root_a())]
#[case("Root {A }", parse_node::enum_root_a())]
#[case("Root { A }", parse_node::enum_root_a())]
fn test_parse_enum_single(#[case] input: &str, #[case] node: ParseNode) {
    assert_that!(parse_enum(input).unwrap()).is_equal_to(("", node));
}

#[rstest]
#[case("Root {A,B}", parse_node::enum_root_ab())]
#[case("Root { A B }", parse_node::enum_root_ab())]
#[case("Root { A { B } }", parse_node::enum_root_a_b())]
#[case("Root { A { B } C }", parse_node::enum_root_a_b_up_c())]
#[case("Root { A { B }, C }", parse_node::enum_root_a_b_up_c())]
fn test_parse_enum_variants(#[case] input: &str, #[case] node: ParseNode) {
    assert_that!(parse_enum(input).unwrap()).is_equal_to(("", node));
}

#[rstest]
#[case::just_a_comma("Root {,}", ParseNode::enumeration("Root", [ ]))]
#[case::mora_commas("Root {,,,,}", ParseNode::enumeration("Root", [ ]))]
#[case::comma_after_variant("Root {A,}", parse_node::enum_root_a())]
#[case::comma_before_variant("Root {,A}", parse_node::enum_root_a())]
#[case::comma_between_variants("Root {A,B}", parse_node::enum_root_ab())]
fn test_parse_enum_optional_commas(#[case] input: &str, #[case] node: ParseNode) {
    assert_that!(parse_enum(input).unwrap()).is_equal_to(("", node));
}

#[rstest]
#[case("Root []", ParseNode::list_empty("Root"))]
#[case("Root[]", ParseNode::list_empty("Root"))]
#[case("Root[A]", parse_node::list_root_a())]
#[case("Root[A,B]", parse_node::list_root_ab())]
fn test_parse_list(#[case] input: &str, #[case] node: ParseNode) {
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
#[case("//Comment", ParseNode::comment("Comment"))]
#[case("// Comment", ParseNode::comment("Comment"))]
fn test_parse_node_with_comments(#[case] input: &str, #[case] comment: ParseNode) {
    assert_that!(parse_node(input)).is_ok_containing(("", comment));
}

#[rstest]
fn test_parse_node_messy_example() {
    let input = "Root [ A { B, C [D E {F G}] H } I J ]";
    assert_debug_snapshot!(parse_node(input));
}

#[rstest]
fn test_parse_list_incomplete() {
    assert_compact_debug_snapshot!(parse_list("Root [ A"), @r#"Err(Error(Error { input: "", code: Tag })) "#);
}

#[rstest]
fn test_parse_enum_incomplete() {
    assert_compact_debug_snapshot!(parse_enum("Root { A"), @r#"Err(Error(Error { input: "", code: Tag }))"#);
}

#[rstest]
#[case("A", vec![ParseNode::singleton("A")])]
#[case("A B", vec![ParseNode::singleton("A"), ParseNode::singleton("B")] )]
#[case("A,B", vec![ParseNode::singleton("A"), ParseNode::singleton("B")] )]
#[case("A, B", vec![ParseNode::singleton("A"), ParseNode::singleton("B")] )]
#[case("A{B}", vec![ParseNode::enumeration("A", [ ParseNode::singleton("B") ])] )]
#[case("A{B,C D,}", vec![ParseNode::enumeration("A", [
    ParseNode::singleton("B"),
    ParseNode::singleton("C"),
    ParseNode::singleton("D")
])])]
#[case("A{B{C{D{E{F}}}}}", vec![ParseNode::enumeration("A", [
    ParseNode::enumeration("B", [
        ParseNode::enumeration("C", [
            ParseNode::enumeration("D", [
                ParseNode::enumeration("E", [
                    ParseNode::singleton("F")
                ])
            ])
        ])
    ])
])])]
#[ case("//A//{ B C }", vec![ ParseNode::comment("A//{ B C }") ])]
#[ case("A//{ B C }", vec![
    ParseNode::singleton("A"),
    ParseNode::comment("{ B C }")
])]
fn test_parse_config(#[case] input: &str, #[case] expected: Vec<ParseNode>) {
    assert_that!(parse_config(input))
        .named(&format!("\"{input}\""))
        .is_ok()
        .is_equal_to(("", expected));
}

#[rstest]
#[case("A//\n{ B C }", vec![
    ParseNode::singleton("A"),
    ParseNode::comment("")
], "{ B C }")]
#[case("A {\n // B\n C\n}...", vec![ ParseNode::enumeration("A", [
    ParseNode::comment("B"),
    ParseNode::singleton("C")
]) ], "...")]
fn test_parse_config_incomplete(
    #[case] input: &str,
    #[case] expected: Vec<ParseNode>,
    #[case] rest: &str,
) {
    assert_that!(parse_config(input))
        .named(&format!("\"{input}\""))
        .is_ok()
        .is_equal_to((rest, expected));
}

#[rstest]
#[case("Root, Root2", vec![
    ParseNode::singleton("Root"),
    ParseNode::singleton("Root2"),
])]
fn test_parse_states_file(#[case] input: &str, #[case] expected: Vec<ParseNode>) {
    assert_that!(parse_states_text(input))
        .is_ok()
        .is_equal_to(expected);
}

#[rstest]
#[case("Main", ParseNode::singleton("Main"))]
#[case("Main{}", ParseNode::Enum("Main".into(), Default::default()))]
#[case("Main{A}", ParseNode::enumeration("Main", [ParseNode::singleton("A")]))]
#[case("Main{A,B}", ParseNode::enumeration("Main", [
    ParseNode::singleton("A"),
    ParseNode::singleton("B"),
]))]
fn test_parse_node_try_from_str(#[case] input: &str, #[case] expected: ParseNode) {
    assert_that!(ParseNode::try_from(input)).is_ok_containing(expected);
}
