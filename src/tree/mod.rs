mod node;
use node::Node;

#[derive(Debug)]
pub struct TrieTree {
  root: Box<Node>,
}

impl TrieTree {
  pub fn new() -> TrieTree {
    TrieTree {
      root: Box::new(Node::new()),
    }
  }

  pub fn insert(&mut self, word: &str) {
    let mut curr_node: *mut Node = &mut *self.root;
    unsafe {
      for ch in word.chars() {
        if !(*curr_node).contains_key(&ch) {
          (*curr_node).insert(&ch, Box::new(Node::new()));
        }
        curr_node = (*curr_node).next_mut(&ch);
      }
      (*curr_node).mark_as_a_word();
    }
  }

  fn get_prefix_end(&self, word: &str) -> Option<*const Node> {
    let mut curr_node: *const Node = &*self.root;
    for ch in word.chars() {
      unsafe {
        if (*curr_node).contains_key(&ch) {
          curr_node = (*curr_node).next(&ch);
        } else {
          return None;
        }
      }
    }
    Some(curr_node)
  }

  pub fn contains(&self, word: &str) -> bool {
    if let Some(node) = self.get_prefix_end(word) {
      unsafe {
        if (*node).is_a_word() {
          true
        } else {
          false
        }
      }
    } else {
      false
    }
  }

  pub fn contains_prefix(&self, word: &str) -> bool {
    if self.get_prefix_end(word).is_some() {
      true
    } else {
      false
    }
  }

  // pub fn top_n(&self, prefix: &str, max_n: usize) -> Option<Vec<String>> {
  //   if let Some(node) = self.get_prefix_end(prefix) {
  //     let mut r = vec![];
  //     return Some(r);
  //   } else {
  //     None
  //   }
  // }
}
