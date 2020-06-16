// ------------------------------------------------------------------------
// Copyright 2020 github.com/chuling <meetchuling@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0
// ------------------------------------------------------------------------

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
                if (*t_node).is_a_seq() {
                    (*t_node).unmark();
                    if !(*t_node).is_a_seq() {
                        let has_alive_child = || -> bool {
                            // dfs
                            let mut to_visit = vec![t_node];
                            while let Some(exp_node) = to_visit.pop() {
                                to_visit.extend((*exp_node).get_all_leaves_mut());
                                if (*exp_node).cannot_be_deleted() {
                                    return true;
                                }
                            }
                            false
                        };
                        if has_alive_child() {
                            return;
                        }
                        // delete
                        let mut curr_node: *mut Node<T> = &mut *self.root;
                        let mut last_alive_node = curr_node;
                        let mut to_del_ch = &seq[0];
                        for ch in seq.iter() {
                            if (*curr_node).cannot_be_deleted() {
                                last_alive_node = curr_node;
                                to_del_ch = ch;
                            }
                            curr_node = (*curr_node).key_next_mut(ch);
                        }
                        (*last_alive_node).key_destroy(to_del_ch);
                    }
                }
            }
        }
    }

    fn get_prefix_end(&self, seq: &[T]) -> Option<*const Node<T>> {
        assert!(seq.len() != 0);
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
        assert!(seq.len() != 0);
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

    // pub fn key_cnt(&self, seq: &[T]) -> Option<usize> {
    //     if seq.len() == 0 {
    //         return None;
    //     }
    //     if let Some(node) = self.get_prefix_end(seq) {
    //         unsafe {
    //             if (*node).is_a_seq() {
    //                 Some((*node).get_elem_cnt())
    //             } else {
    //                 None
    //             }
    //         }
    //     } else {
    //         None
    //     }
    // }

    pub fn prefix_vaild(&self, seq: &[T]) -> bool {
        if seq.len() == 0 {
            return false;
        }
        self.get_prefix_end(seq).is_some()
    }

    pub fn prefix_match<'a>(&self, seq: &'a [T]) -> Option<Vec<Vec<&'a T>>> {
        if seq.len() == 0 {
            return None;
        }
        if let Some(node) = self.get_prefix_end(seq) {
            let mut r = vec![];
            let mut tail_seq: Vec<&T> = vec![];
            // dfs
            unsafe {
                let mut to_visit = vec![Some((seq.last().unwrap(), node))];
                while let Some(record) = to_visit.pop() {
                    if let Some((ch, leaf)) = record {
                        to_visit.push(None);
                        to_visit.extend((*leaf).get_all_leaves().into_iter().map(|v| Some(v)));
                        tail_seq.push(ch);
                        if (*leaf).is_a_seq() {
                            r.push(
                                [seq[..seq.len() - 1].iter().collect(), tail_seq.clone()].concat(),
                            );
                        }
                    } else {
                        tail_seq.pop();
                    }
                }
            }
            return Some(r);
        } else {
            None
        }
    }
}
