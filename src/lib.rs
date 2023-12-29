mod avl;
mod node;
use std::cmp::Ordering;

use node::*;

mod map;
pub use map::*;

pub use avl::*;

pub trait Nearness {
    fn nearer<'a>(&'a self, other: &'a Self, target: &Self) -> &'a Self;
    fn farther<'a>(&'a self, other: &'a Self, target: &Self) -> &'a Self;
}

macro_rules! impl_nearer_signed {
    ($tp:ty) => {
        impl Nearness for $tp {
            fn nearer<'a>(&'a self, other: &'a Self, target: &Self) -> &'a Self {
                if <$tp>::abs(self - target) < <$tp>::abs(other - target) {
                    self
                } else {
                    other
                }
            }
            fn farther<'a>(&'a self, other: &'a Self, target: &Self) -> &'a Self {
                if <$tp>::abs(self - target) > <$tp>::abs(other - target) {
                    self
                } else {
                    other
                }
            }
        }
    };
}


pub struct Pair<K, V> {
    key: K,
    val: V,
}

impl<K: Ord, V> PartialEq for Pair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.key.cmp(&other.key), Ordering::Equal)
    }
}

impl<K: Ord, V> Eq for Pair<K, V> {}

impl<K: Ord, V> Ord for Pair<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl<K: Ord, V> PartialOrd for Pair<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.key.cmp(&other.key))
    }
}

macro_rules! impl_nearer_unsigned {
    ($tp:ty) => {
        impl Nearness for $tp {
            fn nearer<'a>(&'a self, other: &'a Self, target: &Self) -> &'a Self {
                if self < target && other < target {
                    if self > other {
                        self
                    } else {
                        other
                    }
                } else if self > target && other > target {
                    if self < other {
                        self
                    } else {
                        other
                    }
                } else {
                    let (larger, smaller) = if self > target {
                        (self, other)
                    } else {
                        (other, self)
                    };
                    if larger - target < target - smaller {
                        larger
                    } else {
                        smaller
                    }
                }
            }
            fn farther<'a>(&'a self, other: &'a Self, target: &Self) -> &'a Self {
                if self < target && other < target {
                    if self > other {
                        other
                    } else {
                        self
                    }
                } else if self > target && other > target {
                    if self < other {
                        other
                    } else {
                        self
                    }
                } else {
                    let (larger, smaller) = if self > target {
                        (self, other)
                    } else {
                        (other, self)
                    };
                    if larger - target > target - smaller {
                        larger
                    } else {
                        smaller
                    }
                }
            }
        }
    };
}

impl_nearer_signed!(isize);
impl_nearer_signed!(i128);
impl_nearer_signed!(i64);
impl_nearer_signed!(i32);
impl_nearer_signed!(i16);
impl_nearer_signed!(i8);

impl_nearer_unsigned!(usize);
impl_nearer_unsigned!(u128);
impl_nearer_unsigned!(u64);
impl_nearer_unsigned!(u32);
impl_nearer_unsigned!(u16);
impl_nearer_unsigned!(u8);
