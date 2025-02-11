use std::rc::Rc;

use nom::{
    IResult, Parser, branch::alt, bytes::complete::*, character::complete::*,
    combinator::recognize, multi::many0, sequence::*,
};

use crate::{
    nodes::{Node, NodeData},
    tokens::{Comment, Identifier, Token},
};
pub fn comment(input: &str) -> IResult<&str, Comment<'_>> {
    delimited(
        pair(tag("//"), space0),
        not_line_ending,
        pair(space0, line_ending),
    )
    .parse(input)
    .map_result(|c| c.trim_end().into())
}

pub fn identifier(input: &str) -> IResult<&str, Identifier<'_>> {
    recognize(pair(
        take_till1(|c: char| !c.is_uppercase()),
        take_while(|c: char| c.is_alphanumeric() || c == '_'),
    ))
    .parse(input)
    .map_result(Identifier::from)
}

fn skip_to<P, T, O, E>(parser: P) -> impl Parser<T, Output = O, Error = E>
where
    T: nom::Input,
    P: Parser<T, Output = O, Error = E>,
    <T as nom::Input>::Item: nom::AsChar,
    E: nom::error::ParseError<T>,
{
    preceded(multispace0, parser)
}

pub fn separator(input: &str) -> IResult<&str, Token<'_>> {
    skip_to(tag(","))
        .parse(input)
        .map_result(|_| Token::Separator)
}

pub fn open_enum(input: &str) -> IResult<&str, Token<'_>> {
    skip_to(tag("{"))
        .parse(input)
        .map_result(|_| Token::OpenEnum)
}

pub fn close_enum(input: &str) -> IResult<&str, Token<'_>> {
    skip_to(tag("}"))
        .parse(input)
        .map_result(|_| Token::CloseEnum)
}

pub fn open_list(input: &str) -> IResult<&str, Token<'_>> {
    skip_to(tag("["))
        .parse(input)
        .map_result(|_| Token::OpenList)
}

pub fn close_list(input: &str) -> IResult<&str, Token<'_>> {
    skip_to(tag("]"))
        .parse(input)
        .map_result(|_| Token::CloseList)
}

pub trait MapResult<'a, I, O1, O2> {
    fn map_result(self, callback: impl Fn(O1) -> O2) -> IResult<I, O2>;
}

impl<I, O1, O2> MapResult<'_, I, O1, O2> for IResult<I, O1> {
    fn map_result(self, callback: impl Fn(O1) -> O2) -> IResult<I, O2> {
        self.map(|(rest, result)| (rest, callback(result)))
    }
}

pub fn parse_config(input: &str) -> IResult<&str, Vec<Node>> {
    many0(alt((
        parse_node,
        terminated(parse_node, separator),
        preceded(separator, parse_node),
    )))
    .parse(input)
}

pub fn parse_node(input: &str) -> IResult<&str, Node> {
    alt((parse_enum, parse_list, parse_singleton)).parse(input)
}

pub fn parse_singleton(input: &str) -> IResult<&str, Node> {
    skip_to(identifier)
        .parse(input)
        .map_result(NodeData::from)
        .map_result(Node::Singleton)
}

pub fn parse_enum(input: &str) -> IResult<&str, Node> {
    let (input, name) = skip_to(identifier).parse(input)?;
    let (input, children) =
        skip_to(preceded(open_enum, parse_elements_until(close_enum))).parse(input)?;
    Ok((input, Node::Enum(name.into(), children)))
}

pub fn parse_list(input: &str) -> IResult<&str, Node> {
    let (input, name) = skip_to(identifier).parse(input)?;
    let (input, children) =
        skip_to(preceded(open_list, parse_elements_until(close_list))).parse(input)?;
    Ok((input, Node::List(name.into(), children)))
}
pub fn parse_elements_until<'a>(
    until: impl Fn(&str) -> IResult<&str, Token<'_>> + Copy,
) -> impl Fn(&str) -> IResult<&str, Vec<Rc<Node>>> {
    move |input: &str| {
        terminated(
            // 0 or more elements, separated by whitespace or a comma
            many0(alt((
                parse_node,
                terminated(parse_node, separator),
                preceded(separator, parse_node),
            ))),
            // then the closing token
            until,
        )
        .parse(input)
        .map_result(|nodes| nodes.into_iter().map(Rc::new).collect())
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use rstest::rstest;
    use speculoos::prelude::*;

    use super::*;

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
    #[case("//Hello\n", "Hello")]
    #[case("// Hello\n", "Hello")]
    #[case("// Hello \n", "Hello")]
    #[case("// Hello, how are you? \n", "Hello, how are you?")]
    fn test_comment(#[case] input: &str, #[case] expected: &str) {
        assert_that!(comment(input))
            .is_ok()
            .map(|(_, token)| token)
            .is_equal_to(Comment::from(expected));
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
            .map(|(_, token)| token)
            .is_equal_to(expected);
    }

    #[rstest]
    #[case("Name,", ",", Node::singleton("Name"))]
    #[case("  Name,", ",", Node::singleton("Name"))]
    #[case("First, Second", ", Second", Node::singleton("First"))]
    fn test_parse_singleton(#[case] input: &str, #[case] rest: &str, #[case] node: Node) {
        assert_that!(parse_singleton(input).unwrap()).is_equal_to((rest, node));
    }

    #[rstest]
    fn test_parse_enum_empty() {
        assert_debug_snapshot!(parse_enum("Name{}").unwrap(), @r#"
        (
            "",
            Enum(
                NodeData {
                    name: "Name",
                    parent: None,
                },
                [],
            ),
        )
        "#);
        assert_debug_snapshot!(parse_enum("Name {}").unwrap(), @r#"
        (
            "",
            Enum(
                NodeData {
                    name: "Name",
                    parent: None,
                },
                [],
            ),
        )
        "#);
    }

    #[rstest]
    #[case("Name {A}",  Node::enumeration("Name", [Node::singleton("A")]))]
    #[case("Name { A}",  Node::enumeration("Name", [Node::singleton("A")]))]
    #[case("Name {A }",  Node::enumeration("Name", [Node::singleton("A")]))]
    #[case("Name { A }",  Node::enumeration("Name", [Node::singleton("A")]))]
    fn test_parse_enum_single(#[case] input: &str, #[case] node: Node) {
        assert_that!(parse_enum(input).unwrap()).is_equal_to(("", node));
    }

    #[rstest]
    #[case("Name {A,B}", Node::enumeration("Name", [
        Node::singleton("A"),
        Node::singleton("B")
    ]))]
    #[case("Name { A B }",  Node::enumeration("Name", [
        Node::singleton("A"),
        Node::singleton("B")
    ]))]
    #[case("Name { A { B } }",  Node::enumeration("Name", [
        Node::enumeration("A", [Node::singleton("B")])
    ]))]
    #[case("Name { A { B }, C }",  Node::enumeration("Name", [
        Node::enumeration("A", [Node::singleton("B")]),
        Node::singleton("C")
    ]))]
    fn test_parse_enum_variants(#[case] input: &str, #[case] node: Node) {
        assert_that!(parse_enum(input).unwrap()).is_equal_to(("", node));
    }

    #[rstest]
    #[case("Name []", Node::list_empty("Name"))]
    #[case("Name[]", Node::list_empty("Name"))]
    #[case("Name[A]",  Node::list("Name", [Node::singleton("A")]))]
    #[case("Name[A,B]",  Node::list("Name", [
        Node::singleton("A"),
        Node::singleton("B"),
    ]))]
    fn test_parse_list(#[case] input: &str, #[case] node: Node) {
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
    #[case("Name [ A { B, C [D E {F G}] H } I J ]")]
    fn test_parse_node(#[case] input: &str) {
        set_snapshot_suffix!(
            "{}",
            input
                .replace(" ", "_")
                .replace("{", "bo")
                .replace("}", "bc")
                .replace("[", "po")
                .replace("]", "pc")
        );

        assert_debug_snapshot!(parse_node(input));
    }
    #[rstest]
    fn test_parse_list_incomplete() {
        assert_debug_snapshot!(parse_list("Name [ A"), @r#"
        Err(
            Error(
                Error {
                    input: "",
                    code: Tag,
                },
            ),
        )
        "#);
    }

    #[rstest]
    fn test_parse_enum_incomplete() {
        assert_debug_snapshot!(parse_enum("Name { A"), @r#"
        Err(
            Error(
                Error {
                    input: "",
                    code: Tag,
                },
            ),
        )
        "#);
    }
}
