use std::rc::Rc;

use crate::models::StateNode;
use crate::parsing::ParseNode;

pub trait SubTree {
    type Node;
    fn get_tree_size(&self) -> usize;
    fn into_nodes(self) -> Vec<Self::Node>;
}

impl SubTree for StateNode {
    type Node = Rc<Self>;
    fn get_tree_size(&self) -> usize {
        match self {
            StateNode::Singleton(_) => 1,
            StateNode::Enum(_, children) | StateNode::List(_, children) => {
                children
                    .iter()
                    .map(|child| child.get_tree_size())
                    .sum::<usize>()
                    + 1
            }
        }
    }
    fn into_nodes(self) -> Vec<Self::Node> {
        let mut children = self.children();
        let mut nodes = vec![Rc::new(self)];
        nodes.append(&mut children);

        nodes
    }
}

impl SubTree for ParseNode<'_> {
    type Node = Self;
    fn get_tree_size(&self) -> usize {
        match self {
            ParseNode::Comment(_) => 1,
            ParseNode::Singleton(_) => 1,
            ParseNode::Enum(_, children) | ParseNode::List(_, children) => {
                children
                    .iter()
                    .map(|child| child.get_tree_size())
                    .sum::<usize>()
                    + 1
            }
        }
    }
    fn into_nodes(self) -> Vec<Self::Node> {
        let mut children = self.children();
        let mut nodes = vec![self];
        nodes.append(&mut children);

        nodes
    }
}
