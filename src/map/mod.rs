use crate::{AVL, Pair};

pub struct BTreeMap<A, B> {
    avl: AVL<Pair<A, B>>,
}

impl<K, V> BTreeMap<K, V> {
    pub fn new() -> Self {
        Self { avl: AVL::new() }
    }
}

impl<K: Ord, V> BTreeMap<K, V> {
    pub fn contains_key(&self, key: &K) -> bool {
        self.avl.root.as_ref().map(|v| v.contains_by(|en| key.cmp(&en.key))).unwrap_or(false)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.avl.root.as_ref().map(|v| v.get_by(|en| key.cmp(&en.key))).unwrap_or(None).map(|v| &v.val)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.avl.root.as_mut().map(|v| v.get_mut_by(|en| key.cmp(&en.key))).unwrap_or(None).map(|v| &mut v.val)
    }

    pub fn insert(&mut self, key: K, val: V) -> bool {
        let entry = Pair { key, val };
        self.avl.insert_distinct(entry)
    }

    pub fn len(&self) -> usize {
        self.avl.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Pair<K, V>> {
        self.avl.increasing()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Pair<K, V>> {
        self.avl.into_increasing()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.avl.increasing().map(|v| &v.key)
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.avl.increasing().map(|v| &v.val)
    }

    pub fn into_keys(self) -> impl Iterator<Item = K> {
        self.avl.into_increasing().map(|v| v.key)
    }

    pub fn into_values(self) -> impl Iterator<Item = V> {
        self.avl.into_increasing().map(|v| v.val)
    }

    pub fn increasing(&self) -> impl Iterator<Item = &Pair<K, V>> {
        self.avl.increasing()
    }

    pub fn decreasing(&self) -> impl Iterator<Item = &Pair<K, V>> {
        self.avl.decreasing()
    }
}
