use std::rc::Rc;

use iter_tools::Itertools;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeData {
    Orphan { name: String },
    Child { name: String, parent: Rc<Node> },
}

impl NodeData {
    pub fn is_orphan(&self) -> bool {
        matches!(self, NodeData::Orphan { .. })
    }
    pub fn new<S: ToString>(name: S, parent: Option<Rc<Node>>) -> Self {
        let name = name.to_string();
        match parent {
            Some(parent) => NodeData::Child { name, parent },
            None => NodeData::Orphan { name },
        }
    }
    pub fn name(&self) -> &str {
        match self {
            NodeData::Orphan { name } => name,
            NodeData::Child { name, .. } => name,
        }
    }
    pub fn computed_name(&self) -> String {
        match self {
            NodeData::Orphan { name } => name.clone(),
            NodeData::Child { name, parent } => {
                format!("{}{}", parent.computed_name(), name)
            }
        }
    }
    pub fn try_set_parent(&mut self, parent: Rc<Node>) -> Result<(), String> {
        if let NodeData::Orphan { name } = self {
            *self = NodeData::Child {
                name: name.clone(),
                parent,
            };
            Ok(())
        } else {
            Err("Node already has a parent".to_string())
        }
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
    #[cfg(test)]
    pub fn list<N: Into<Rc<Node>>, V: IntoIterator<Item = N>, D: Into<NodeData>>(
        data: D,
        variants: V,
    ) -> Self {
        Self::List(
            data.into(),
            variants.into_iter().map(Into::into).collect_vec(),
        )
    }
    #[cfg(test)]
    pub fn list_empty<N: Into<NodeData>>(name: N) -> Self {
        Self::List(name.into(), Vec::new())
    }
    pub fn children(&self) -> Vec<Rc<Node>> {
        match self {
            Node::Enum(_, variants) | Node::List(_, variants) => variants.clone(),
            _ => Default::default(),
        }
    }
    pub fn data(&self) -> &NodeData {
        match self {
            Node::Singleton(data) | Node::Enum(data, _) | Node::List(data, _) => data,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            Node::Singleton(data) | Node::Enum(data, _) | Node::List(data, _) => data.name(),
        }
    }
    pub fn computed_name(&self) -> String {
        match self {
            Node::Singleton(name) | Node::Enum(name, _) | Node::List(name, _) => {
                name.computed_name()
            }
        }
    }
    pub fn try_set_parent(&mut self, parent: Rc<Node>) -> Result<(), String> {
        match self {
            Node::Singleton(data) => data.try_set_parent(parent),
            Node::Enum(data, _) | Node::List(data, _) => data.try_set_parent(parent),
        }
    }
}
