mod node;
use node::Node;

use std::cmp::Eq;
use std::hash::Hash;

#[derive(Debug)]
pub struct TrieTree<T: Hash + Eq + Clone> {
  root: Box<Node<T>>,
}

impl<T: Hash + Eq + Clone> TrieTree<T> {
  pub fn new() -> TrieTree<T> {
    TrieTree {
      root: Box::new(Node::new()),
    }
  }

  pub fn insert(&mut self, word: &[T]) {
    let mut curr_node: *mut Node<T> = &mut *self.root;
    unsafe {
      for ch in word.iter() {
        if !(*curr_node).contains_key(ch) {
          (*curr_node).key_alloc(ch);
        }
        curr_node = (*curr_node).next_mut(ch);
      }
      (*curr_node).mark_as_a_word();
    }
  }

  fn get_prefix_end(&self, word: &[T]) -> Option<*const Node<T>> {
    let mut curr_node: *const Node<T> = &*self.root;
    for ch in word.iter() {
      unsafe {
        if (*curr_node).contains_key(ch) {
          curr_node = (*curr_node).next(ch);
        } else {
          return None;
        }
      }
    }
    Some(curr_node)
  }

  pub fn contains(&self, word: &[T]) -> bool {
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

  pub fn contains_prefix(&self, word: &[T]) -> bool {
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
