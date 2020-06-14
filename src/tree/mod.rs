mod node;
use node::TreeNode;

#[derive(Debug)]
pub struct Tree<T> {
    root: Option<TreeNode<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { root: None }
    }

    pub fn insert(&mut self, val: T) {
        if self.root.is_none() {
            self.root = Some(TreeNode::new(val));
        }
    }
}
