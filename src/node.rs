use std::fmt::Debug;

#[derive(Debug, Clone)]
pub(crate) struct Node<T> {
    pub(crate) height: i32,
    pub(crate) val: T,
    pub(crate) left: Option<Box<Node<T>>>,
    pub(crate) right: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> Node<T> {
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

impl<T: PartialEq + Ord> Node<T> {
    pub(crate) fn delete(mut self: Box<Node<T>>, val: &T) -> (bool, Option<Box<Node<T>>>) {
        let (con, rv) = if val == &self.val {
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

        if let Some(mut res) = rv {
            let bf = res.bf();
            if bf > 1 {
                if let Some(left) = &mut res.left {
                    if left.bf() < 0 {
                        left.rotate_left();
                    }

                    res.rotate_right();
                }
            } else if bf < -1 {
                if let Some(right) = &mut res.right {
                    if right.bf() > 0 {
                        right.rotate_right();
                    }
                    res.rotate_left();
                }
            }
            (con, Some(res))
        } else {
            (con, None)
        }
    }
}

impl Node<i32> {
    pub fn nearest(self: &Box<Self>, val: i32) -> i32 {
        if val == self.val {
            self.val
        } else if val < self.val {
            if let Some(left) = &self.left {
                let ll = left.nearest(val);
                if i32::abs(ll - val) < i32::abs(self.val - val) {
                    ll
                } else {
                    self.val
                }
            } else {
                self.val
            }
        } else {
            if let Some(right) = &self.right {
                let rr = right.nearest(val);
                if i32::abs(rr - val) < i32::abs(self.val - val) {
                    rr
                } else {
                    self.val
                }
            } else {
                self.val
            }
        }
    }
}
