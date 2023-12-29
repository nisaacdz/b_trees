use crate::Node;
use std::collections::LinkedList;

pub struct IntoIter<T> {
    pub(crate) nodes: LinkedList<Box<Node<T>>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let mut node = self.nodes.pop_front();
        if let Some(node) = &mut node {
            if let Some(l_node) = node.left.take() {
                self.nodes.push_back(l_node);
            }
            if let Some(r_node) = node.right.take() {
                self.nodes.push_back(r_node);
            }
        }
        node.map(|n| n.val)
    }
}


pub struct Iter<'a, T> {
    pub(crate) nodes: LinkedList<&'a Box<Node<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.nodes.pop_front();
        if let Some(node) = node {
            if let Some(l_node) = &node.left {
                self.nodes.push_back(l_node);
            }
            if let Some(r_node) = &node.right {
                self.nodes.push_back(r_node);
            }
        }
        node.map(|n| &n.val)
    }
}
