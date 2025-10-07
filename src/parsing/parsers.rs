use nom::branch::alt;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::*;
use nom::{IResult, Parser};

use crate::parsing::{Comment, Identifier, ParseNode, Token};

pub fn parse_comment(input: &str) -> IResult<&str, ParseNode<'_>> {
    comment(input).map_result(ParseNode::Comment)
}

pub fn comment(input: &str) -> IResult<&str, Comment<'_>> {
    use nom::combinator::eof;

    delimited(skip_to(tag("//")), not_line_ending, alt((eof, line_ending)))
        .parse(input)
        .map_result(|c| c.trim().into())
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

pub fn separator(input: &str) -> IResult<&str, Token> {
    skip_to(tag(","))
        .parse(input)
        .map_result(|_| Token::Separator)
}

pub fn open_enum(input: &str) -> IResult<&str, Token> {
    skip_to(tag("{"))
        .parse(input)
        .map_result(|_| Token::OpenEnum)
}

pub fn close_enum(input: &str) -> IResult<&str, Token> {
    skip_to(tag("}"))
        .parse(input)
        .map_result(|_| Token::CloseEnum)
}

pub fn open_list(input: &str) -> IResult<&str, Token> {
    skip_to(tag("["))
        .parse(input)
        .map_result(|_| Token::OpenList)
}

pub fn close_list(input: &str) -> IResult<&str, Token> {
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

pub fn parse_config(input: &str) -> IResult<&str, Vec<ParseNode<'_>>> {
    many0(delimited(many0(separator), parse_node, many0(separator))).parse(input)
}

pub fn parse_node(input: &'_ str) -> IResult<&'_ str, ParseNode<'_>> {
    alt((parse_enum, parse_list, parse_comment, parse_singleton)).parse(input)
}

pub fn parse_singleton(input: &'_ str) -> IResult<&'_ str, ParseNode<'_>> {
    skip_to(identifier)
        .parse(input)
        .map_result(ParseNode::singleton)
}

pub fn parse_enum(input: &str) -> IResult<&str, ParseNode<'_>> {
    let (input, name) = terminated(skip_to(identifier), skip_to(open_enum)).parse(input)?;
    let (input, children) = skip_to(parse_elements_until(close_enum)).parse(input)?;
    Ok((input, ParseNode::Enum(name, children)))
}

pub fn parse_list(input: &'_ str) -> IResult<&'_ str, ParseNode<'_>> {
    let (input, name) = skip_to(identifier).parse(input)?;
    let (input, children) =
        skip_to(preceded(open_list, parse_elements_until(close_list))).parse(input)?;
    Ok((input, ParseNode::List(name, children)))
}

pub fn parse_elements_until<'a>(
    until: impl Fn(&'a str) -> IResult<&'a str, Token> + Copy,
) -> impl Fn(&'a str) -> IResult<&'a str, Vec<ParseNode<'a>>> {
    move |input: &'a str| {
        delimited(
            // ignore any leading whitespace and commas
            many0(separator),
            // 0 or more elements, ignoring whitespace and commas
            many0(alt((terminated(parse_node, many0(separator)),))),
            // then expect the closing token
            until,
        )
        .parse(input)
    }
}
