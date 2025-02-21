use derive_more::{Deref, From};

#[derive(Debug, PartialEq)]
pub enum Token {
    Separator,
    OpenEnum,
    CloseEnum,
    #[cfg(feature = "lists")]
    OpenList,
    #[cfg(feature = "lists")]
    CloseList,
}

#[derive(Debug, PartialEq, From, Deref)]
pub struct Identifier<'a>(&'a str);

impl std::fmt::Display for Identifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "comments")]
#[derive(Debug, PartialEq, From, Deref)]
pub struct Comment<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub enum ParseNode<'a> {
    Singleton(Identifier<'a>),
    Enum(Identifier<'a>, Vec<ParseNode<'a>>),
    #[cfg(feature = "lists")]
    List(Identifier<'a>, Vec<ParseNode<'a>>),
    #[cfg(feature = "comments")]
    Comment(Comment<'a>),
}

impl<'a> ParseNode<'a> {
    pub fn singleton<I: Into<Identifier<'a>>>(name: I) -> Self {
        Self::Singleton(name.into())
    }
    pub fn enumeration<I: Into<Identifier<'a>>, V: IntoIterator<Item = ParseNode<'a>>>(
        name: I,
        variants: V,
    ) -> Self {
        Self::Enum(name.into(), variants.into_iter().collect())
    }
    #[cfg(all(test, feature = "lists"))]
    pub fn list_empty<I: Into<Identifier<'a>>>(name: I) -> Self {
        Self::List(name.into(), vec![])
    }
    #[cfg(all(test, feature = "lists"))]
    pub fn list<I: Into<Identifier<'a>>, V: IntoIterator<Item = ParseNode<'a>>>(
        name: I,
        variants: V,
    ) -> Self {
        Self::List(name.into(), variants.into_iter().collect())
    }
    #[cfg(all(test, feature = "comments"))]
    pub fn comment<C: Into<Comment<'a>>>(name: C) -> Self {
        Self::Comment(name.into())
    }
}

impl<'a> TryFrom<&'a str> for ParseNode<'a> {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        use crate::parsing::parse_node;
        parse_node(s).map(|(_, node)| node)
    }
}
