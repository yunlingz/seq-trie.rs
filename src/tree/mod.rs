mod node;
use node::Node;

use std::cmp::Eq;
use std::fmt;
use std::hash::Hash;

#[derive(Debug)]
pub struct TrieTree<T: Hash + Eq + Clone> {
  root: Box<Node<T>>,
}

impl<T: Hash + Eq + Clone + fmt::Debug> TrieTree<T> {
  pub fn new() -> TrieTree<T> {
    TrieTree {
      root: Box::new(Node::new()),
    }
  }

  pub fn insert(&mut self, seq: &[T]) {
    if seq.len() == 0 {
      return;
    }
    let mut curr_node: *mut Node<T> = &mut *self.root;
    unsafe {
      for ch in seq.iter() {
        if !(*curr_node).contains_key(ch) {
          (*curr_node).key_alloc(ch);
        }
        curr_node = (*curr_node).key_next_mut(ch);
      }
      (*curr_node).mark();
    }
  }

  pub fn remove(&mut self, seq: &[T]) {
    if seq.len() == 0 {
      return;
    }
    if let Some(t_node) = self.get_prefix_end_mut(seq) {
      unsafe {
        if (*t_node).is_a_word() {
          (*t_node).unmark();
          if !(*t_node).is_a_word() {
            let has_alive_child = |root_node: *mut Node<T>| -> bool {
              // dfs
              let mut to_visit = vec![root_node];
              while let Some(exp_node) = to_visit.pop() {
                to_visit.extend((*exp_node).get_all_leaves_mut());
                if (*exp_node).is_a_word() || (*exp_node).get_child_cnt() > 1 {
                  return true;
                }
              }
              false
            };
            if has_alive_child(t_node) {
              return;
            }
            println!("start to delete");
            // delete
            let mut curr_node: *mut Node<T> = &mut *self.root;
            let mut last_alive_node = curr_node;
            let mut to_del_ch = &seq[0];
            for ch in seq.iter() {
              if (*curr_node).is_a_word() || (*curr_node).get_child_cnt() > 1 {
                last_alive_node = curr_node;
                to_del_ch = ch;
              }
              curr_node = (*curr_node).key_next_mut(ch);
            }
            println!("ch = {:?}", to_del_ch);
            (*last_alive_node).key_destroy(to_del_ch);
          }
        }
      }
    }
  }

  fn get_prefix_end(&self, seq: &[T]) -> Option<*const Node<T>> {
    let mut curr_node: *const Node<T> = &*self.root;
    for ch in seq.iter() {
      unsafe {
        if (*curr_node).contains_key(ch) {
          curr_node = (*curr_node).key_next(ch);
        } else {
          return None;
        }
      }
    }
    Some(curr_node)
  }

  fn get_prefix_end_mut(&mut self, seq: &[T]) -> Option<*mut Node<T>> {
    let mut curr_node: *mut Node<T> = &mut *self.root;
    for ch in seq.iter() {
      unsafe {
        if (*curr_node).contains_key(ch) {
          curr_node = (*curr_node).key_next_mut(ch);
        } else {
          return None;
        }
      }
    }
    Some(curr_node)
  }

  pub fn contains(&self, seq: &[T]) -> bool {
    if seq.len() == 0 {
      return false;
    }
    if let Some(node) = self.get_prefix_end(seq) {
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

  pub fn contains_prefix(&self, seq: &[T]) -> bool {
    if seq.len() == 0 {
      return false;
    }
    self.get_prefix_end(seq).is_some()
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
