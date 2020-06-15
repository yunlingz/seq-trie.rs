mod node;
use node::Node;

#[derive(Debug)]
pub struct Tree {
  root: Box<Node>,
}

impl Tree {
  pub fn new() -> Tree {
    Tree {
      root: Box::new(Node::new()),
    }
  }

  pub fn insert(&mut self, word: &String) {
    let mut curr_node: *mut Node = &mut *self.root;
    for ch in word.chars() {
      unsafe {
        if !(*curr_node).contains_key(&ch) {
          (*curr_node).insert(&ch, Box::new(Node::new()));
        }
        curr_node = (*curr_node).get(&ch);
      }
    }
  }
}
