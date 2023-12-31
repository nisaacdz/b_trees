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

use std::{collections::LinkedList, fmt::Debug, cmp::Ordering};

use crate::Nearness;

use self::iters::{IntoIncreasing, IntoDecreasing};

use super::Node;
use iters::{Decreasing, Increasing, Levels, IntoIter, Iter};

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
    pub(crate) root: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> AVL<T> {
    /// Creates and returns a new AVL tree
    #[inline]
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    /// Returns the number of nodes in this AVL tree. This operation has a strict time complexity of `O(1)`
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn height(&self) -> usize {
        match &self.root {
            Some(root) => root.height as usize,
            None => 0,
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
        self.root = None;
    }

    #[inline]
    pub fn levels(&self) -> impl Iterator<Item = impl Iterator<Item = Option<&T>>> {
        Levels {
            h_left: self.height(),
            cur: LinkedList::from_iter(Some(self.root.as_ref())),
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        Iter { nodes: LinkedList::from_iter(self.root.as_ref()) }
    }

    /// Returns an in-order traversal iterator over the elements in the binary tree.
    /// This ensures the returned values are in an `increasing` order according to their implementation of the `Ord` trait.
    ///
    /// Although this implementation does not make the iterator **lazy**, that is, initializing this iterator uses time complexity of O(log(n)), it makes the average time complexity of `next` be amortized O(1) with worst case scenario of O(log(n)) and ratio of average case to worst case is 1: log(n).
    /// More generally speaking, this implementation performs better than other implementations and also uses no extra space.
    #[inline]
    pub fn increasing(&self) -> impl Iterator<Item = &T> {
        Increasing::new(self.root.as_ref())
    }

    #[inline]
    pub fn into_increasing(self) -> impl Iterator<Item = T> {
        IntoIncreasing::new(self.root)
    }

    #[inline]
    pub fn into_decreasing(self) -> impl Iterator<Item = T> {
        IntoDecreasing::new(self.root)
    }

    #[inline]
    pub fn decreasing(&self) -> impl Iterator<Item = &T> {
        Decreasing::new(self.root.as_ref())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
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
    pub fn insert_distinct(&mut self, val: T) -> bool {
        if let Some(root) = &mut self.root {
            if root.insert_distinct(val) {
                self.len += 1;
                true
            } else {
                false
            }
        } else {
            self.root = Some(Box::new(Node::new(val)));
            self.len += 1;
            true
        }
    }

    #[inline]
    pub fn remove_by(&mut self, f: impl FnMut(&T) -> Ordering) -> Option<T> {
        let mut res = None;
        self.root = if let Some(root) = self.root.take() {
            let (v, val) = root.remove_by(f);
            res = v;
            val
        } else {
            None
        };
        if res.is_some() {
            self.len -= 1
        }
        res
    }

    #[inline]
    pub fn remove(&mut self, val: &T) -> Option<T> {
        let mut res = None;
        self.root = if let Some(root) = self.root.take() {
            let (v, val) = root.delete(&val);
            res = v;
            val
        } else {
            None
        };
        if res.is_some() {
            self.len -= 1
        }
        res
    }

    #[inline]
    pub fn delete(&mut self, val: &T) -> bool {
        let mut con = false;
        self.root = if let Some(root) = self.root.take() {
            let (v, val) = root.delete(&val);
            con = v.is_some();
            val
        } else {
            None
        };
        if con {
            self.len -= 1
        }
        con
    }

    #[inline]
    pub fn union(mut self, mut other: Self) -> Self {
        if self.len() > other.len() {
            for val in other {
                self.insert(val);
            }
            self
        } else {
            for val in self {
                other.insert(val);
            }
            other
        }
    }

    #[inline]
    pub fn contains(&self, target: &T) -> bool {
        self.root.as_ref().map(|n| n.contains(target)).unwrap_or(false)
    }


    #[inline]
    pub fn max(&self) -> Option<&T> {
        self.root.as_ref().map(|r| r.find_max())
    }

    /// traverses the binary search tree and returns the minimum value.
    #[inline]
    pub fn min(&self) -> Option<&T> {
        self.root.as_ref().map(|r| r.find_min())
    }

    #[inline]
    pub fn nearest_to<'a, F>(&'a self, target: &'a T, by: F) -> Option<&'a T>
    where
        F: 'static + Fn(&'a T, &'a T) -> &'a T,
        T: 'a,
    {
        self.root.as_ref().map(|r| r.nearest_to(target, &by))
    }

    #[inline]
    pub fn farthest_to<'a, F>(&'a self, target: &'a T, by: F) -> Option<&'a T>
    where
        F: 'static + Fn(&'a T, &'a T) -> &'a T,
        T: 'a,
    {
        self.root.as_ref().map(|r| r.farthest_to(target, &by))
    }
    
    pub fn greater_than<'a>(&'a self, lower: &'a T) -> impl Iterator<Item = &'a T> {
        self.increasing().skip_while(|&v| v <= lower)
    }

    pub fn less_than<'a>(&'a self, upper: &'a T) -> impl Iterator<Item = &'a T> {
        self.decreasing().skip_while(|&v| v >= upper)
    }
}

impl<T: Ord + Nearness> AVL<T> {
    #[inline]
    pub fn nearest<'a>(&'a self, target: &'a T) -> Option<&'a T> {
        self.root
            .as_ref()
            .map(|r| r.nearest_to(target, &move |a, b| T::nearer(a, b, target)))
    }

    #[inline]
    pub fn farthest<'a>(&'a self, target: &'a T) -> Option<&'a T> {
        self.root
            .as_ref()
            .map(|r| r.farthest_to(target, &move |a, b| T::farther(a, b, target)))
    }
}

impl<T> IntoIterator for AVL<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            nodes: LinkedList::from_iter(self.root)
        }
    }
}

impl<T: Ord> FromIterator<T> for AVL<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut avl = Self::new();
        for val in iter {
            avl.insert(val)
        }
        avl
    }
}
