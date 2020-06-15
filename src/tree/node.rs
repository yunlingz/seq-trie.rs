use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
  child: HashMap<char, Box<Node>>,
}

impl Node {
  pub fn new() -> Node {
    Node {
      child: HashMap::new(),
    }
  }

  pub fn contains_key(&self, ch: &char) -> bool {
    self.child.contains_key(ch)
  }

  pub fn get(&mut self, ch: &char) -> *mut Node {
    let node = self.child.get_mut(&ch).unwrap();
    return &mut **node;
  }

  pub fn insert(&mut self, ch: &char, node: Box<Node>) {
    self.child.insert(*ch, node);
  }
}
