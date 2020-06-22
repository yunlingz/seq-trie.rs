// ------------------------------------------------------------------------
// Copyright 2020 github.com/chuling <meetchuling@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------------------------------------

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Node<T: Hash + Eq + Clone> {
  child: HashMap<T, Box<Node<T>>>,
  is_seq: bool,
}

impl<T: Hash + Eq + Clone> Node<T> {
  pub fn new() -> Node<T> {
    Node {
      child: HashMap::new(),
      is_seq: false,
    }
  }

  // pub fn key_destroy(&mut self, ch: &T) {
  //     self.child.remove(ch);
  // }

  // pub fn get_all_leaves_mut(&mut self) -> Vec<(&T, &mut Box<Node<T>>)> {
  //     self.child.iter_mut().collect()
  // }

  pub fn get_all_leaves(&self) -> Vec<(&T, &Box<Node<T>>)> {
    self.child.iter().collect()
  }

  pub fn contains_key(&self, ch: &T) -> bool {
    self.child.contains_key(ch)
  }

  pub fn key_next(&self, ch: &T) -> &Box<Node<T>> {
    self.child.get(ch).unwrap()
  }

  pub fn key_next_mut(&mut self, ch: &T) -> &mut Box<Node<T>> {
    self.child.get_mut(ch).unwrap()
  }

  pub fn key_alloc(&mut self, ch: T) {
    self.child.insert(ch, Box::new(Node::new()));
  }

  // pub fn cannot_be_deleted(self: &Box<Self>) -> bool {
  //     let mut to_visit = vec![self];
  //     while let Some(exp_node) = to_visit.pop() {
  //         to_visit.extend(exp_node.get_all_leaves().into_iter().map(|(_, v)| v));
  //         if exp_node.is_a_seq() {
  //             return true;
  //         }
  //     }
  //     false
  // }

  pub fn is_a_seq(&self) -> bool {
    self.is_seq
  }

  pub fn mark(&mut self) -> bool {
    const TARGET_VAL: bool = true;
    if self.is_seq == TARGET_VAL {
      false
    } else {
      self.is_seq = TARGET_VAL;
      true
    }
  }

  pub fn unmark(&mut self) -> bool {
    const TARGET_VAL: bool = false;
    if self.is_seq == TARGET_VAL {
      false
    } else {
      self.is_seq = TARGET_VAL;
      true
    }
  }
}
