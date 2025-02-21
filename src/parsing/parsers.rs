use nom::{
    IResult, Parser, branch::alt, bytes::complete::*, character::complete::*,
    combinator::recognize, multi::many0, sequence::*,
};

#[cfg(feature = "comments")]
use crate::tokens::Comment;
use crate::tokens::{Identifier, ParseNode, Token};

#[cfg(feature = "comments")]
pub fn parse_comment(input: &str) -> IResult<&str, ParseNode<'_>> {
    comment(input).map_result(ParseNode::Comment)
}

/// ```rust
/// # #[cfg(feature = "comments")] {
/// # use bevy_state_plugin_generator::comment;
/// assert_eq!(comment("//comment"), Ok(("", "comment".into())));
/// assert_eq!(comment("// comment "), Ok(("", "comment".into())));
/// assert_eq!(comment(" // comment "), Ok(("", "comment".into())));
/// assert_eq!(comment("//\ncomment "), Ok(("comment ", "".into())));
/// # }
/// ```
#[cfg(feature = "comments")]
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

#[cfg(feature = "lists")]
pub fn open_list(input: &str) -> IResult<&str, Token> {
    skip_to(tag("["))
        .parse(input)
        .map_result(|_| Token::OpenList)
}

#[cfg(feature = "lists")]
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

pub(super) fn parse_config(input: &str) -> IResult<&str, Vec<ParseNode<'_>>> {
    many0(delimited(many0(separator), parse_node, many0(separator))).parse(input)
}

pub fn parse_states_file<'a>(
    input: &'a str,
    root_state_name: &'a str,
) -> Result<ParseNode<'a>, String> {
    parse_config(input)
        .map_err(|e| format!("{:?}", e))
        .map(|(_, nodes)| {
            if nodes.is_empty() {
                ParseNode::singleton(root_state_name)
            } else {
                ParseNode::enumeration(root_state_name, nodes)
            }
        })
}

/// Validate that the input is a valid states file
/// ```rust
/// # use bevy_state_plugin_generator::config_is_valid;
/// assert!(config_is_valid("Name, Name2"));
/// assert!(config_is_valid("Name { A, B }"));
/// assert!(config_is_valid("Name { ,,,, }  "));
/// assert!(!config_is_valid("{]"));
/// ```
pub fn config_is_valid(input: &str) -> bool {
    parse_config(input)
        .map(|(rest, _)| rest.trim().is_empty())
        .unwrap_or(false)
}

pub fn parse_node(input: &str) -> IResult<&str, ParseNode> {
    alt((
        parse_enum,
        #[cfg(feature = "lists")]
        parse_list,
        #[cfg(feature = "comments")]
        parse_comment,
        parse_singleton,
    ))
    .parse(input)
}

pub fn parse_singleton(input: &str) -> IResult<&str, ParseNode> {
    skip_to(identifier)
        .parse(input)
        .map_result(ParseNode::singleton)
}

pub fn parse_enum(input: &str) -> IResult<&str, ParseNode<'_>> {
    let (input, name) = terminated(skip_to(identifier), skip_to(open_enum)).parse(input)?;
    let (input, children) = skip_to(parse_elements_until(close_enum)).parse(input)?;
    Ok((input, ParseNode::Enum(name, children)))
}

#[cfg(feature = "lists")]
pub fn parse_list(input: &str) -> IResult<&str, ParseNode> {
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
