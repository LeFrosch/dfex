use std::collections::BTreeSet;
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct MultiId {
    pub(super) ids: BTreeSet<usize>,
}

impl MultiId {
    pub fn new(id: usize) -> Self {
        Self { ids: BTreeSet::from([id]) }
    }

    pub fn insert(&mut self, id: usize) {
        self.ids.insert(id);
    }

    pub fn extend(&mut self, other: &MultiId) {
        self.ids.extend(&other.ids);
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.ids.iter()
    }
}

impl Default for MultiId {
    fn default() -> Self {
        Self { ids: BTreeSet::new() }
    }
}
