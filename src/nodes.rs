use std::rc::Rc;

use iter_tools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Singleton(String),
    Enum(String, Vec<Rc<Node>>),
    List(String, Vec<Rc<Node>>),
}

impl Node {
    pub fn singleton<S: ToString>(name: S) -> Self {
        Self::Singleton(name.to_string())
    }
    pub fn enumeration<N: Into<Rc<Node>>, V: IntoIterator<Item = N>, S: ToString>(
        name: S,
        variants: V,
    ) -> Self {
        Self::Enum(
            name.to_string(),
            variants.into_iter().map(Into::into).collect_vec(),
        )
    }
    #[cfg(test)]
    pub fn list<N: Into<Rc<Node>>, V: IntoIterator<Item = N>, S: ToString>(
        name: S,
        variants: V,
    ) -> Self {
        Self::List(
            name.to_string(),
            variants.into_iter().map(Into::into).collect_vec(),
        )
    }
    #[cfg(test)]
    pub fn list_empty<N: ToString>(name: N) -> Self {
        Self::List(name.to_string(), Vec::new())
    }
    pub fn name(&self) -> &str {
        match self {
            Node::Singleton(name) | Node::Enum(name, _) | Node::List(name, _) => name,
        }
    }
}
