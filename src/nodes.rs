use std::{
    any::{type_name, type_name_of_val},
    rc::Rc,
};

use iter_tools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub struct NodeData {
    pub name: String,
    pub parent: Option<Rc<Node>>,
}

impl NodeData {
    pub fn new<S: ToString>(name: S, parent: Option<Rc<Node>>) -> Self {
        NodeData {
            name: name.to_string(),
            parent,
        }
    }
    pub fn computed_name(&self) -> String {
        format!(
            "{}{}",
            self.parent
                .as_ref()
                .map(|parent| parent.computed_name())
                .unwrap_or_default(),
            self.name
        )
    }
}

impl<S: ToString> From<S> for NodeData {
    fn from(value: S) -> Self {
        Self::new(value, None)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Singleton(NodeData),
    Enum(NodeData, Vec<Rc<Node>>),
    List(NodeData, Vec<Rc<Node>>),
}

impl Node {
    pub fn singleton<D: Into<NodeData>>(data: D) -> Self {
        Self::Singleton(data.into())
    }
    pub fn enumeration<N: Into<Rc<Node>>, V: IntoIterator<Item = N>, D: Into<NodeData>>(
        data: D,
        variants: V,
    ) -> Self {
        Self::Enum(
            data.into(),
            variants.into_iter().map(Into::into).collect_vec(),
        )
    }
    pub fn list<N: Into<Rc<Node>>, V: IntoIterator<Item = N>, D: Into<NodeData>>(
        data: D,
        variants: V,
    ) -> Self {
        Self::List(
            data.into(),
            variants.into_iter().map(Into::into).collect_vec(),
        )
    }
    pub fn list_empty<N: Into<NodeData>>(name: N) -> Self {
        Self::List(name.into(), Vec::new())
    }
    pub fn own_name(&self) -> String {
        match self {
            Node::Singleton(data) | Node::Enum(data, _) | Node::List(data, _) => data.name.clone(),
        }
    }
    pub fn computed_name(&self) -> String {
        match self {
            Node::Singleton(name) | Node::Enum(name, _) | Node::List(name, _) => {
                name.computed_name()
            }
        }
    }
    pub fn prepend_variant(&mut self, variant: Rc<Node>) {
        let to_add = variant.clone();
        match self {
            Node::Singleton(name) => {
                *self = Node::Enum(name.clone(), vec![variant]);
            }
            Node::Enum(_, variants) => {
                variants.insert(0, variant);
            }
            Node::List(_, variants) => {
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
