use std::fmt::Display;

use derive_more::{Deref, From};

#[derive(Debug, PartialEq)]
pub enum Token {
    Separator,
    OpenEnum,
    CloseEnum,
    OpenList,
    CloseList,
}

#[derive(Debug, PartialEq, From, Deref)]
pub struct Identifier<'a>(&'a str);

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, From, Deref)]
pub struct Comment<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub enum ParseNode<'a> {
    Singleton(Identifier<'a>),
    Enum(Identifier<'a>, Vec<ParseNode<'a>>),
    List(Identifier<'a>, Vec<ParseNode<'a>>),
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
}
