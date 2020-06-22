// ------------------------------------------------------------------------
// Copyright 2020 github.com/chuling <meetchuling@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------------------------------------

mod node;
use node::Node;

use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct TrieTree<T: Hash + Eq + Clone> {
  root: Box<Node<T>>,
  is_dirty: bool,
}

impl<'a, T: 'a + Hash + Eq + Clone> TrieTree<T> {
  pub fn new() -> TrieTree<T> {
    TrieTree {
      root: Box::new(Node::new()),
      is_dirty: false,
    }
  }

  pub fn insert(&mut self, seq: Vec<T>) -> bool {
    if seq.len() == 0 {
      return false;
    }
    let mut curr_node = &mut self.root;
    for ch in seq.into_iter() {
      let ch_record = ch.clone();
      if !curr_node.contains_key(&ch) {
        curr_node.key_alloc(ch);
      }
      curr_node = curr_node.key_next_mut(&ch_record);
    }
    curr_node.mark()
  }

  pub fn insert_iterator<U>(&mut self, mut collect: U) -> bool
  where
    U: Iterator<Item = Vec<T>>,
  {
    while let Some(item) = collect.next() {
      self.insert(item);
    }
    false
  }

  pub fn remove(&mut self, seq: &[T]) -> bool {
    if seq.len() == 0 {
      return false;
    }
    let r = if let Some(t_node) = self.get_prefix_end_mut(seq) {
      t_node.unmark()
    } else {
      false
    };
    if r {
      self.is_dirty = true;
    }
    r
  }

  pub fn requires_gc(&self) -> bool {
    self.is_dirty
  }

  pub fn gc(&mut self) -> bool {
    if !self.requires_gc() {
      return false;
    }
    // --- gc start ---
    // TODO
    // --- gc end ---
    self.is_dirty = false;
    true
  }

  pub fn contains(&self, seq: &[T]) -> bool {
    self.get_prefix_end(seq).is_some()
  }

  fn get_prefix_end(&self, seq: &[T]) -> Option<&Box<Node<T>>> {
    assert!(seq.len() != 0);
    let mut curr_node = &self.root;
    for ch in seq.iter() {
      if curr_node.contains_key(ch) {
        curr_node = curr_node.key_next(ch);
      } else {
        return None;
      }
    }
    Some(curr_node)
  }

  fn get_prefix_end_mut(&mut self, seq: &[T]) -> Option<&mut Box<Node<T>>> {
    assert!(seq.len() != 0);
    let mut curr_node = &mut self.root;
    for ch in seq.iter() {
      if curr_node.contains_key(ch) {
        curr_node = curr_node.key_next_mut(ch);
      } else {
        return None;
      }
    }
    Some(curr_node)
  }

  pub fn prefix_vaild(&self, seq: &[T]) -> bool {
    if seq.len() == 0 {
      return false;
    }
    self.get_prefix_end(seq).is_some()
  }

  pub fn prefix_match_maxn(
    &'a self,
    seq: &'a [T],
    maxn: usize,
  ) -> Option<Vec<Vec<&'a T>>> {
    if seq.len() == 0 {
      return None;
    }
    if maxn == 0 {
      return Some(vec![]);
    }
    if let Some(node) = self.get_prefix_end(seq) {
      let mut r = vec![];
      let mut tail_seq: Vec<&T> = vec![];
      // dfs
      let mut to_visit = vec![Some((seq.last().unwrap(), node))];
      while let Some(record) = to_visit.pop() {
        if let Some((ch, leaf)) = record {
          to_visit.push(None);
          to_visit.extend(leaf.get_all_leaves().into_iter().map(|v| Some(v)));
          tail_seq.push(ch);
          if leaf.is_a_seq() {
            r.push(
              [seq[..seq.len() - 1].iter().collect(), tail_seq.clone()]
                .concat(),
            );
            if r.len() == maxn {
              break;
            }
          }
        } else {
          tail_seq.pop();
        }
      }
      return Some(r);
    } else {
      None
    }
  }

  pub fn prefix_match(&'a self, seq: &'a [T]) -> Option<Vec<Vec<&'a T>>> {
    self.prefix_match_maxn(seq, usize::MAX)
  }
}
