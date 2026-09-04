#[derive(Debug)]
pub struct StateTree {}

pub trait Builder {
    type Output;
    fn build(self) -> Self::Output;
}

struct StateTreeBuilder {}

impl Builder for StateTreeBuilder {
    type Output = StateTree;

    fn build(self) -> Self::Output {
        StateTree {}
    }
}

impl StateTree {
    pub fn builder() -> impl Builder<Output = Self> {
        StateTreeBuilder {}
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use rstest::*;

    use super::*;

    #[rstest]
    fn main() {
        let b = StateTree::builder();
        let tree = b.build();
        assert_debug_snapshot!(tree, @"StateTree");
    }
}
