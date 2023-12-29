use crate::node::Node;

pub struct GreaterThan<'a, T> {
    lower: &'a T,
}

impl<'a, T> GreaterThan<'a, T> {
    pub(crate) fn new(root: Option<&'a Box<Node<T>>>, lower: &'a T) -> Self {
        todo!()
    }
}

impl<'a, T> Iterator for GreaterThan<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct LessThan<'a, T> {
    upper: &'a T,
}

impl<'a, T> LessThan<'a, T> {
    pub(crate) fn new(root: Option<&'a Box<Node<T>>>, upper: &'a T) -> Self {
        todo!()
    }
}

impl<'a, T> Iterator for LessThan<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}