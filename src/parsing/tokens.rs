use derive_more::{Deref, From};

#[derive(Debug, PartialEq)]
pub enum Token {
    Separator,
    OpenEnum,
    CloseEnum,
    OpenList,
    CloseList,
}

#[derive(Debug, Deref, From, PartialEq)]
pub struct Identifier<'a>(&'a str);

impl std::fmt::Display for Identifier<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deref, From, PartialEq)]
pub struct Comment<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub enum ParseNode<'a> {
    Singleton(Identifier<'a>),
    Enum(Identifier<'a>, Vec<ParseNode<'a>>),
    List(Identifier<'a>, Vec<ParseNode<'a>>),
    Comment(Comment<'a>),
    #[cfg(feature = "directives")]
    Directive(Directive<'a>),
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
}

impl<'a> TryFrom<&'a str> for ParseNode<'a> {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        crate::parsing::parse_node(s).map(|(_, node)| node)
    }
}

#[cfg(feature = "directives")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Directive<'a>(ConfigProperty, &'a str);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConfigProperty {
    /// name of the struct that implements [bevy::plugin::Plugin]
    PluginName,
    /// name of the root enum/struct that represents the game state
    StateName,
    /// name of the module that contains sub-states
    StatesModuleName,
    /// naming scheme for the generated states
    Scheme,
    /// add additional traits to the derive list
    AdditionDerives,
}
