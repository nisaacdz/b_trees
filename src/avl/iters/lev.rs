use std::collections::LinkedList;

use crate::node::Node;

pub struct Levels<'a, T> {
    pub(crate) h_left: usize,
    pub(crate) cur: LinkedList<Option<&'a Box<Node<T>>>>,
}

pub struct Level<'a, T> {
    ll: LinkedList<Option<&'a Box<Node<T>>>>,
}

impl<'a, T: std::fmt::Debug> std::fmt::Debug for Level<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.ll, f)
    }
}

impl<'a, T> Iterator for Level<'a, T> {
    type Item = Option<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.ll.pop_front().map(|v| v.map(|v| &v.val))
    }
}

impl<'a, T> Iterator for Levels<'a, T> {
    type Item = Level<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.h_left > 0 {
            let mut newlist = LinkedList::new();
            for &node in &self.cur {
                newlist.push_back(node.map(|n| n.left.as_ref()).flatten());
                newlist.push_back(node.map(|n| n.right.as_ref()).flatten());
            }
            self.h_left -= 1;
            Some(Level {
                ll: std::mem::replace(&mut self.cur, newlist),
            })
        } else {
            None
        }
    }
}
