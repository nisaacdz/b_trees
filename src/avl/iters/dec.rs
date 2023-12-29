use crate::node::Node;

struct FakeNode<'a, T> {
    parent: Option<Box<FakeNode<'a, T>>>,
    node: &'a Box<Node<T>>,
}

impl<'a, T> FakeNode<'a, T> {
    fn init(node: &'a Box<Node<T>>) -> Self {
        let mut parent = None;
        let mut cur = FakeNode { parent, node };

        while let Some(right_node) = &cur.node.right {
            parent = Some(Box::new(cur));
            cur = FakeNode {
                node: right_node,
                parent,
            };
        }
        cur
    }
    fn new(node: &'a Box<Node<T>>, mut parent: Option<Box<FakeNode<'a, T>>>) -> Self {
        let mut cur = FakeNode { parent, node };
        while let Some(right_node) = &cur.node.right {
            parent = Some(Box::new(cur));
            cur = FakeNode {
                node: right_node,
                parent,
            };
        }
        cur
    }
}

pub struct Decreasing<'a, T> {
    node: Option<Box<FakeNode<'a, T>>>,
}

impl<'a, T> Decreasing<'a, T> {
    pub(crate) fn new(node: Option<&'a Box<Node<T>>>) -> Self {
        match node {
            None => Self { node: None },
            Some(node) => {
                let node = Some(Box::new(FakeNode::init(node)));
                Self { node }
            }
        }
    }
}

impl<'a, T> Iterator for Decreasing<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.node {
            None => None,
            Some(node) => {
                let rv = &node.node.val;
                self.node = if let Some(l_node) = &node.node.left {
                    let parent = node.parent.take();
                    Some(Box::new(FakeNode::new(l_node, parent)))
                } else {
                    node.parent.take()
                };
                Some(rv)
            }
        }
    }
}

struct FakeNode2<T> {
    parent: Option<Box<FakeNode2<T>>>,
    node: Box<Node<T>>,
}

impl<T> FakeNode2<T> {
    fn init(node: Box<Node<T>>) -> Self {
        let mut parent = None;
        let mut cur = FakeNode2 { parent, node };

        while let Some(right_node) = cur.node.right.take() {
            parent = Some(Box::new(cur));
            cur = FakeNode2 {
                node: right_node,
                parent,
            };
        }
        cur
    }
    fn new(node: Box<Node<T>>, mut parent: Option<Box<FakeNode2<T>>>) -> Self {
        let mut cur = FakeNode2 { parent, node };
        while let Some(right_node) = cur.node.right.take() {
            parent = Some(Box::new(cur));
            cur = FakeNode2 {
                node: right_node,
                parent,
            };
        }
        cur
    }
}


pub struct IntoDecreasing<T> {
    node: Option<Box<FakeNode2<T>>>,
}

impl<T> IntoDecreasing<T> {
    pub(crate) fn new(node: Option<Box<Node<T>>>) -> Self {
        match node {
            None => Self { node: None },
            Some(node) => {
                let node = Some(Box::new(FakeNode2::init(node)));
                Self { node }
            }
        }
    }
}

impl<T> Iterator for IntoDecreasing<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let cur_node = self.node.take();
        match cur_node {
            None => None,
            Some(mut node) => {
                let rv = if let Some(l_node) = node.node.left.take() {
                    let res = Some(node.node.val);
                    let parent = node.parent.take();
                    self.node = Some(Box::new(FakeNode2::new(l_node, parent)));
                    res
                } else {
                    let res = Some(node.node.val);
                    self.node = node.parent.take();
                    res
                };
                rv
            }
        }
    }
}