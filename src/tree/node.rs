use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Node<T: Hash + Eq + Clone> {
  child: HashMap<T, Box<Node<T>>>,
  elem_cnt: usize,
}

impl<T: Hash + Eq + Clone> Node<T> {
  pub fn new() -> Node<T> {
    Node {
      child: HashMap::new(),
      elem_cnt: 0,
    }
  }

  pub fn key_destroy(&mut self, ch: &T) {
    self.child.remove(ch);
  }

  pub fn get_all_leaves_mut(&mut self) -> Vec<*mut Node<T>> {
    let mut r = Vec::<*mut Node<T>>::new();
    for (_, v) in self.child.iter_mut() {
      r.push(&mut **v);
    }
    r
  }

  pub fn get_all_leaves(&self) -> Vec<(&T, *const Node<T>)> {
    let mut r = Vec::<(&T, *const Node<T>)>::new();
    for (k, v) in self.child.iter() {
      r.push((k, &**v))
    }
    r
  }

  pub fn contains_key(&self, ch: &T) -> bool {
    self.child.contains_key(ch)
  }

  pub fn key_next(&self, ch: &T) -> *const Node<T> {
    let node = self.child.get(ch).unwrap();
    return &**node;
  }

  pub fn key_next_mut(&mut self, ch: &T) -> *mut Node<T> {
    let node = self.child.get_mut(ch).unwrap();
    return &mut **node;
  }

  pub fn key_alloc(&mut self, ch: &T) {
    self.child.insert(ch.clone(), Box::new(Node::new()));
  }

  pub fn cannot_be_deleted(&self) -> bool {
    self.is_a_word() || self.child.keys().len() > 1
  }

  pub fn is_a_word(&self) -> bool {
    self.elem_cnt != 0
  }

  pub fn mark(&mut self) {
    self.elem_cnt += 1;
  }

  pub fn unmark(&mut self) {
    self.elem_cnt -= 1;
  }

  pub fn get_elem_cnt(&self) -> usize {
    self.elem_cnt
  }
}
