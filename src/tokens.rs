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

impl ToString for Identifier<'_> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, PartialEq, From, Deref)]
pub struct Comment<'a>(&'a str);
