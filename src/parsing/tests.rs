use nom::Parser;
use nom::branch::alt;

use super::parsers::*;
use crate::testing::*;
use crate::tokens::*;

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
#[case("Name,", ",", ParseNode::singleton("Name"))]
#[case("  Name,", ",", ParseNode::singleton("Name"))]
#[case("First, Second", ", Second", ParseNode::singleton("First"))]
fn test_parse_singleton(#[case] input: &str, #[case] rest: &str, #[case] node: ParseNode) {
    assert_that!(parse_singleton(input).unwrap()).is_equal_to((rest, node));
}

#[rstest]
fn test_parse_enum_empty() {
    assert_compact_debug_snapshot!(parse_enum("Name{}").unwrap(), @r#"("", Enum(Identifier("Name"), []))"#);
    assert_compact_debug_snapshot!(parse_enum("Name {}").unwrap(), @r#"("", Enum(Identifier("Name"), []))"#);
}

#[rstest]
#[case("Name {A}",  ParseNode::enumeration("Name", [ParseNode::singleton("A")]))]
#[case("Name { A}",  ParseNode::enumeration("Name", [ParseNode::singleton("A")]))]
#[case("Name {A }",  ParseNode::enumeration("Name", [ParseNode::singleton("A")]))]
#[case("Name { A }",  ParseNode::enumeration("Name", [ParseNode::singleton("A")]))]
fn test_parse_enum_single(#[case] input: &str, #[case] node: ParseNode) {
    assert_that!(parse_enum(input).unwrap()).is_equal_to(("", node));
}

#[rstest]
#[case("Name {A,B}", ParseNode::enumeration("Name", [
    ParseNode::singleton("A"),
    ParseNode::singleton("B")
]))]
#[case("Name { A B }",  ParseNode::enumeration("Name", [
    ParseNode::singleton("A"),
    ParseNode::singleton("B")
]))]
#[case("Name { A { B } }",  ParseNode::enumeration("Name", [
    ParseNode::enumeration("A", [ParseNode::singleton("B")])
]))]
#[case("Name { A { B }, C }",  ParseNode::enumeration("Name", [
    ParseNode::enumeration("A", [ParseNode::singleton("B")]),
    ParseNode::singleton("C")
]))]
fn test_parse_enum_variants(#[case] input: &str, #[case] node: ParseNode) {
    assert_that!(parse_enum(input).unwrap()).is_equal_to(("", node));
}

#[rstest]
#[case::just_a_comma("Name {,}", ParseNode::enumeration("Name", [ ]))]
#[case::mora_commas("Name {,,,,}", ParseNode::enumeration("Name", [ ]))]
#[case::comma_after_variant("Name {A,}",
    ParseNode::enumeration("Name", [ ParseNode::singleton("A") ]))]
#[case::comma_before_variant("Name {,A}",
    ParseNode::enumeration("Name", [ ParseNode::singleton("A") ]))]
#[case::comma_between_variants("Name {A,B}",
    ParseNode::enumeration("Name", [
        ParseNode::singleton("A"),
        ParseNode::singleton("B")
    ])
)]
fn test_parse_enum_optional_commas(#[case] input: &str, #[case] node: ParseNode) {
    assert_that!(parse_enum(input).unwrap()).is_equal_to(("", node));
}

#[rstest]
#[case("Name []", ParseNode::list_empty("Name"))]
#[case("Name[]", ParseNode::list_empty("Name"))]
#[case("Name[A]",  ParseNode::list("Name", [ParseNode::singleton("A")]))]
#[case("Name[A,B]",  ParseNode::list("Name", [
    ParseNode::singleton("A"),
    ParseNode::singleton("B"),
]))]
fn test_parse_list(#[case] input: &str, #[case] node: ParseNode) {
    assert_that!(parse_list(input).unwrap()).is_equal_to(("", node));
}

macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}

#[rstest]
#[case("Name")]
#[case("Name {A}")]
fn test_parse_node(#[case] input: &str) {
    set_snapshot_suffix!("{}", input.replace(" ", "_"));
    assert_compact_debug_snapshot!(parse_node(input));
}

#[rstest]
fn test_parse_node_nested_enums() {
    let input = "Name { A { B, C {D E {F G}} H } I J }";
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
    let input = "Name [ A { B, C [D E {F G}] H } I J ]";
    assert_debug_snapshot!(parse_node(input));
}

#[rstest]
fn test_parse_list_incomplete() {
    assert_compact_debug_snapshot!(parse_list("Name [ A"), @r#"Err(Error(Error { input: "", code: Tag })) "#);
}

#[rstest]
fn test_parse_enum_incomplete() {
    assert_compact_debug_snapshot!(parse_enum("Name { A"), @r#"Err(Error(Error { input: "", code: Tag }))"#);
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
#[case("Name, Name2", vec![
    ParseNode::singleton("Name"),
    ParseNode::singleton("Name2"),
])]
fn test_parse_states_file(#[case] input: &str, #[case] expected: Vec<ParseNode>) {
    assert_that!(parse_states_text(input))
        .is_ok()
        .is_equal_to(expected);
}
