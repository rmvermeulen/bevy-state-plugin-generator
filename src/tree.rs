use crate::parsing::ParseNode;

pub trait SubTree {
    type Node;
    fn get_tree_size(&self) -> usize;
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
}
