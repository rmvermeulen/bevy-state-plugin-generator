use std::{
    any::{type_name, type_name_of_val},
    rc::Rc,
};

use crate::stack_parser::{self, Node};

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum StateConfig {
    Single(String),
    Enum(String, Vec<Rc<StateConfig>>),
    List(String, Vec<Rc<StateConfig>>),
}

impl StateConfig {
    pub fn name(&self) -> &str {
        match self {
            StateConfig::Single(name) | StateConfig::Enum(name, _) | StateConfig::List(name, _) => {
                name
            }
        }
    }
    pub fn single<S: ToString>(name: S) -> Self {
        Self::Single(name.to_string())
    }
    pub fn list<S: ToString, C: Into<Rc<StateConfig>>, V: IntoIterator<Item = C>>(
        name: S,
        variants: V,
    ) -> Self {
        Self::List(
            name.to_string(),
            variants.into_iter().map(Into::into).collect(),
        )
    }
    pub fn variants<S: ToString, C: Into<Rc<StateConfig>>, V: IntoIterator<Item = C>>(
        name: S,
        variants: V,
    ) -> Self {
        Self::Enum(
            name.to_string(),
            variants.into_iter().map(Into::into).collect(),
        )
    }
    pub fn prepend_variant(&mut self, variant: Rc<StateConfig>) {
        let to_add = variant.clone();
        match self {
            StateConfig::Single(name) => {
                *self = StateConfig::Enum(name.to_string(), vec![variant]);
            }
            StateConfig::Enum(_, variants) => {
                variants.insert(0, variant);
            }
            StateConfig::List(_, variants) => {
                variants.insert(0, variant);
            }
        }
        println!(
            "{}::{}: {:?} (after adding {:?})",
            type_name::<Self>(),
            type_name_of_val(&Self::prepend_variant),
            self,
            to_add
        );
    }
}

impl From<stack_parser::Node> for StateConfig {
    fn from(node: Node) -> Self {
        match node {
            stack_parser::Node::Singleton(name) => StateConfig::single(name),
            stack_parser::Node::Enum(name, children) => StateConfig::variants(
                name,
                children
                    .into_iter()
                    .map(|child| -> StateConfig { (*child).clone().into() }),
            ),
            stack_parser::Node::List(name, children) => StateConfig::list(
                name,
                children
                    .into_iter()
                    .map(|child| -> StateConfig { (*child).clone().into() }),
            ),
        }
    }
}
