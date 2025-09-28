use derive_more::{Deref, From};

use crate::models::StateNode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Separator,
    OpenEnum,
    CloseEnum,
    OpenList,
    CloseList,
}

#[derive(Clone, Copy, Debug, Deref, From, PartialEq)]
pub struct Identifier<'a>(&'a str);

impl std::fmt::Display for Identifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Deref, From, PartialEq)]
pub struct Comment<'a>(&'a str);

#[derive(Clone, Debug, PartialEq)]
pub enum ParseNode<'a> {
    Singleton(Identifier<'a>),
    Enum(Identifier<'a>, Vec<ParseNode<'a>>),
    List(Identifier<'a>, Vec<ParseNode<'a>>),
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
    #[cfg(test)]
    pub fn list_empty<I: Into<Identifier<'a>>>(name: I) -> Self {
        Self::List(name.into(), vec![])
    }
    #[cfg(test)]
    pub fn list<I: Into<Identifier<'a>>, V: IntoIterator<Item = ParseNode<'a>>>(
        name: I,
        variants: V,
    ) -> Self {
        Self::List(name.into(), variants.into_iter().collect())
    }
    #[cfg(test)]
    pub fn comment<C: Into<Comment<'a>>>(name: C) -> Self {
        Self::Comment(name.into())
    }
    fn identifier(&self) -> Option<&Identifier<'a>> {
        match self {
            Self::Singleton(id) | Self::Enum(id, _) | Self::List(id, _) => Some(id),
            _ => None,
        }
    }
    pub fn name(&self) -> Option<&str> {
        self.identifier().map(|id| id.0)
    }
    pub fn children(&self) -> Vec<ParseNode<'a>> {
        match self {
            Self::Singleton(_) | Self::Comment(_) => Vec::new(),
            Self::Enum(_, children) => children.clone(),
            Self::List(_, children) => children.clone(),
        }
    }
    pub fn try_into_state_node(self) -> Result<StateNode, ()> {
        self.try_into()
    }
}

impl<'a> TryFrom<&'a str> for ParseNode<'a> {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        crate::parsing::parse_node(s).map(|(_, node)| node)
    }
}
