use std::{fmt::Debug, cmp::Ordering};

#[derive(Debug, Clone)]
pub(crate) struct Node<T> {
    pub(crate) height: i32,
    pub(crate) val: T,
    pub(crate) left: Option<Box<Node<T>>>,
    pub(crate) right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    pub(crate) fn new(val: T) -> Self {
        Node {
            val,
            height: 1,
            left: None,
            right: None,
        }
    }

    /// # Balance Factor
    ///
    /// This function computes and returns the balance factor of the currrent node
    #[inline]
    pub(crate) fn bf(&self) -> i32 {
        self.left.as_ref().map(|l| l.height).unwrap_or(0)
            - self.right.as_ref().map(|r| r.height).unwrap_or(0)
    }
    
    #[inline]
    pub(crate) fn balance(self: &mut Box<Node<T>>) {
        let bf = self.bf();
        if bf > 1 {
            if let Some(left) = &mut self.left {
                if left.bf() < 0 {
                    left.rotate_left();
                }

                self.rotate_right();
            }
        } else if bf < -1 {
            if let Some(right) = &mut self.right {
                if right.bf() > 0 {
                    right.rotate_right();
                }
                self.rotate_left();
            }
        }
    }

    pub(crate) fn find_min(&self) -> &T {
        if let Some(left) = &self.left {
            left.find_min()
        } else {
            &self.val
        }
    }

    pub(crate) fn find_max(&self) -> &T {
        if let Some(right) = &self.right {
            right.find_max()
        } else {
            &self.val
        }
    }

    pub(crate) fn insert_distinct(self: &mut Box<Self>, val: T) -> bool {
        let res = match val.cmp(&self.val) {
            Ordering::Less => if let Some(left) = &mut self.left {
                left.insert_distinct(val)
            } else {
                self.left = Some(Box::new(Node {
                    height: 1,
                    val,
                    left: None,
                    right: None,
                }));
                true
            },
            Ordering::Equal => {
                self.val = val;
                false
            },
            Ordering::Greater => if let Some(right) = &mut self.right {
                right.insert_distinct(val)
            } else {
                self.right = Some(Box::new(Node {
                    height: 1,
                    val,
                    left: None,
                    right: None,
                }));
                true
            },
        };
        self.balance();
        res
    }

    pub(crate) fn insert(self: &mut Box<Self>, val: T) {
        if val < self.val {
            if let Some(left) = &mut self.left {
                left.insert(val)
            } else {
                self.left = Some(Box::new(Node {
                    height: 1,
                    val,
                    left: None,
                    right: None,
                }));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(val);
            } else {
                self.right = Some(Box::new(Node {
                    height: 1,
                    val,
                    left: None,
                    right: None,
                }));
            }
        }
        self.update_height();
        self.balance();
    }
}

impl<T> Node<T> {
    #[inline]
    fn update_height(&mut self) {
        self.height = 1 + i32::max(
            self.left.as_ref().map(|l| l.height).unwrap_or(0),
            self.right.as_ref().map(|r| r.height).unwrap_or(0),
        );
    }

    #[inline]
    fn rotate_left(self: &mut Box<Node<T>>) {
        if let Some(mut new_head) = self.right.take() {
            let head_left = new_head.left.take();
            let mut old_head = std::mem::replace(self, new_head);
            old_head.right = head_left;
            old_head.update_height();
            self.left = Some(old_head);
            self.update_height();
        }
    }

    #[inline]
    fn rotate_right(self: &mut Box<Node<T>>) {
        if let Some(mut new_head) = self.left.take() {
            let head_right = new_head.right.take();
            let mut old_head = std::mem::replace(self, new_head);
            old_head.left = head_right;
            old_head.update_height();
            self.right = Some(old_head);
            self.update_height();
        }
    }
}

impl<T: Ord> Node<T> {
    pub(crate) fn delete(mut self: Box<Node<T>>, val: &T) -> (bool, Option<Box<Node<T>>>) {
        let (con, mut rv) = if val == &self.val {
            match (self.left, self.right) {
                (Some(left), Some(mut right)) => {
                    let mut t_val = &mut right;
                    while let Some(val) = &mut t_val.left {
                        t_val = val;
                    }
                    let new_val = std::mem::replace(&mut t_val.val, self.val);
                    let right = right.delete(&val).1;
                    let left = Some(left);
                    let mut newnode = Box::new(Node {
                        height: 1,
                        val: new_val,
                        left,
                        right,
                    });
                    newnode.update_height();
                    (true, Some(newnode))
                }
                (v, None) | (None, v) => (true, v),
            }
        } else if val > &self.val {
            if let Some(rn) = self.right.take() {
                let (r, rn) = rn.delete(val);
                self.right = rn;
                (r, Some(self))
            } else {
                (false, Some(self))
            }
        } else {
            if let Some(ln) = self.left.take() {
                let (r, ln) = ln.delete(val);
                self.left = ln;
                (r, Some(self))
            } else {
                (false, Some(self))
            }
        };
        rv.as_mut().map(|v| v.balance());
        (con, rv)
    }
    pub(crate) fn nearest_to<'a, F>(&'a self, target: &'a T, by: &F) -> &'a T
    where
        T: 'a,
        F: Fn(&'a T, &'a T) -> &'a T,
    {
        match target.cmp(&self.val) {
            Ordering::Equal => &self.val,
            Ordering::Greater => {
                if let Some(right) = &self.right {
                    by(&self.val, right.nearest_to(target, by))
                } else {
                    &self.val
                }
            }
            Ordering::Less => {
                if let Some(left) = &self.left {
                    by(&self.val, left.nearest_to(target, by))
                } else {
                    &self.val
                }
            }
        }
    }

    pub(crate) fn contains(&self, target: &T) -> bool {
        match target.cmp(&self.val) {
            Ordering::Less => self.left.as_ref().map(|l| l.contains(target)).unwrap_or(false),
            Ordering::Equal => true,
            Ordering::Greater => self.right.as_ref().map(|r| r.contains(target)).unwrap_or(false),
        }
    }

    pub(crate) fn farthest_to<'a, F>(&'a self, target: &'a T, by: &F) -> &'a T
    where
        T: 'a,
        F: Fn(&'a T, &'a T) -> &'a T,
    {
        match target.cmp(&self.val) {
            Ordering::Equal => match (&self.left, &self.right) {
                (Some(left), Some(right)) => {
                    by(left.farthest_to(target, by), right.farthest_to(target, by))
                }
                (Some(only), _) | (_, Some(only)) => only.farthest_to(target, by),
                _ => &self.val,
            },
            Ordering::Greater => {
                if let Some(left) = &self.left {
                    by(&self.val, left.farthest_to(target, by))
                } else {
                    &self.val
                }
            }
            Ordering::Less => {
                if let Some(right) = &self.right {
                    by(&self.val, right.farthest_to(target, by))
                } else {
                    &self.val
                }
            }
        }
    }
}


impl<T> Node<T> {
    pub(crate) fn contains_by(&self, mut f: impl FnMut(&T) -> Ordering) -> bool {
        match f(&self.val) {
            Ordering::Less => self.left.as_ref().map(|l| l.contains_by(f)).unwrap_or(false),
            Ordering::Equal => true,
            Ordering::Greater => self.right.as_ref().map(|r| r.contains_by(f)).unwrap_or(false),
        }
    }

    pub(crate) fn get_by(&self, mut f: impl FnMut(&T) -> Ordering) -> Option<&T> {
        match f(&self.val) {
            Ordering::Less => self.left.as_ref().map(|l| l.get_by(f)).unwrap_or(None),
            Ordering::Equal => Some(&self.val),
            Ordering::Greater => self.right.as_ref().map(|r| r.get_by(f)).unwrap_or(None),
        }
    }

    pub(crate) fn get_mut_by(&mut self, mut f: impl FnMut(&T) -> Ordering) -> Option<&mut T> {
        match f(&self.val) {
            Ordering::Less => self.left.as_mut().map(|l| l.get_mut_by(f)).unwrap_or(None),
            Ordering::Equal => Some(&mut self.val),
            Ordering::Greater => self.right.as_mut().map(|r| r.get_mut_by(f)).unwrap_or(None),
        }
    }
}