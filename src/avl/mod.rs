//! This module contains the implementation of an AVL tree.
//!
//! # Examples
//!
//! ```
//! use avl_tree::AVL;
//!
//! let mut tree = AVL::new();
//!
//! tree.insert(3);
//! tree.insert(1);
//! tree.insert(2);
//!
//! assert_eq!(tree.len(), 3);
//! assert_eq!(tree.height(), 2);
//!
//! let mut iter = tree.iter();
//!
//! assert_eq!(iter.next(), Some(&1));
//! assert_eq!(iter.next(), Some(&2));
//! assert_eq!(iter.next(), Some(&3));
//! assert_eq!(iter.next(), None);
//! ```

use std::{collections::LinkedList, fmt::Debug};

use iters::{Decreasing, Increasing, Levels};
use super::Node;

mod iters;


/// ## Description
///
/// An AVL tree is a self-balancing binary search tree that maintains a height difference of at most 1 
/// between its left and right subtrees. This property ensures that the tree remains balanced, 
/// which in turn guarantees that the time complexities of `insertion`, `deletion`,`lookup` are all `O(log(n))` 
/// Compared to normal binary search trees, AVL trees provide faster lookups and insertions, but 
/// slower deletions. Compared to heaps, AVL trees are more strictly balanced, which makes them 
/// faster for lookups, but slower for insertions and deletions.
///
/// ## Time complexities
///
/// The following table shows the time complexities of various operations in an AVL tree:
///
/// | **Operation** | **Time complexity** |
/// |---------------|---------------------|
/// | Insertion     | O(log n)            |
/// | Deletion      | O(log n)            |
/// | Lookup        | O(log n)            |
///
///

#[derive(Debug, Clone)]
pub struct AVL<T> {
    root: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> AVL<T> {
    #[inline]
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn height(&self) -> usize {
        match &self.root {
            Some(root) => root.height as usize,
            None => 0
        }
    }

    #[inline]
    pub fn levels(&self) -> impl Iterator<Item = impl Iterator<Item = Option<&T>>> {
        Levels { h_left: self.height(), cur: LinkedList::from_iter(Some(self.root.as_ref()))}
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        Increasing::new(self.root.as_ref())
    }

    #[inline]
    pub fn increasing(&self) -> impl Iterator<Item = &T> {
        Increasing::new(self.root.as_ref())
    }

    #[inline]
    pub fn decreasing(&self) -> impl Iterator<Item = &T> {
        Decreasing::new(self.root.as_ref())
    }
}

impl<T: Ord> AVL<T> {
    #[inline]
    pub fn insert(&mut self, val: T) {
        if let Some(root) = &mut self.root {
            root.insert(val);
        } else {
            self.root = Some(Box::new(Node::new(val)))
        }
        self.len += 1
    }

    #[inline]
    pub fn delete(&mut self, val: &T) -> bool {
        let mut con = false;
        self.root = if let Some(root) = self.root.take() {
            let (v, val) = root.delete(&val);
            con = v;
            val
        } else { None };
        if con { self.len -= 1 }
        con
    }

    #[inline]
    pub fn max(&self) -> Option<&T> {
        self.root.as_ref().map(|r| r.find_max())
    }

    #[inline]
    pub fn min(&self) -> Option<&T> {
        self.root.as_ref().map(|r| r.find_min())
    }
}

impl AVL<i32> {
    #[inline]
    pub fn nearest(&self, val: i32) -> Option<i32> {
        self.root.as_ref().map(|r| r.nearest(val))
    }

    #[inline]
    pub fn min_abs_diff(&self, val: i32) -> Option<i32> {
        self.root.as_ref().map(|r| i32::abs(val - r.nearest(val)))
    }
}