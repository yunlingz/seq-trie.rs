#[derive(Debug)]
pub struct TreeNode<T> {
    val: T,
    child: Option<Vec<Box<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> TreeNode<T> {
        TreeNode { val, child: None }
    }
}
