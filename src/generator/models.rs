use std::fmt::{self, Display};

use derive_more::{Deref, From};
use itertools::Itertools;

use super::ToStringWith;

#[derive(Clone, Debug, Deref, From)]
pub(super) struct TypeDefinitions(Vec<StateDef>);

impl TypeDefinitions {
    pub fn inner(self) -> Vec<StateDef> {
        self.0
    }
}

impl ToStringWith for TypeDefinitions {
    fn to_string_indented<S: AsRef<str>>(&self, join: S) -> String {
        let inner = format!("\n{}", join.as_ref());
        let outer = format!("\n{inner}");
        self.0
            .iter()
            .map(|td| td.to_string().lines().join(&inner))
            .join(&outer)
    }
}

impl Display for TypeDefinitions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_indented(""))
    }
}

#[derive(Clone, Debug)]
pub(super) struct StateDef {
    pub parent_name: Option<String>,
    pub typename: String,
    pub source: String,
}

impl Display for StateDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl ToStringWith for StateDef {
    fn to_string_indented<S: AsRef<str>>(&self, join: S) -> String {
        self.source.lines().join(join.as_ref())
    }
}
