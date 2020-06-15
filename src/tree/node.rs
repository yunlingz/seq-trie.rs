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

    pub fn contains_key(&self, ch: &T) -> bool {
        self.child.contains_key(ch)
    }

    pub fn next(&self, ch: &T) -> *const Node<T> {
        let node = self.child.get(ch).unwrap();
        return &**node;
    }

    pub fn next_mut(&mut self, ch: &T) -> *mut Node<T> {
        let node = self.child.get_mut(ch).unwrap();
        return &mut **node;
    }

    pub fn key_alloc(&mut self, ch: &T) {
        self.child.insert(ch.clone(), Box::new(Node::new()));
    }

    pub fn is_a_word(&self) -> bool {
        self.elem_cnt != 0
    }

    pub fn mark_as_a_word(&mut self) {
        self.elem_cnt += 1
    }
}
