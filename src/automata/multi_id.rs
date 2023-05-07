use std::hash::Hash;

#[derive(Hash, Clone)]
pub struct MultiId {
    ids: Vec<usize>,
}

impl MultiId {
    pub fn new(id: usize) -> Self {
        Self { ids: vec![id]}
    }

    pub fn insert(&mut self, id: usize) {
        for i in 0..self.ids.len() {
            if self.ids[i] == id {
                break;
            }
            
            if self.ids[i] > id {
                self.ids.insert(i, id);
                break;
            }
        }

        self.ids.push(id);
    }

    pub fn extend(&mut self, other: &MultiId) {
        let mut buffer = Vec::with_capacity(self.ids.len() + other.ids.len());

        let mut self_ptr = 0;
        let mut other_ptr = 0;

        while self_ptr < self.ids.len() && other_ptr < other.ids.len() {
            match (self.ids[self_ptr], other.ids[other_ptr]) {
                (s, o) if s < o  => {
                    buffer.push(s);
                    self_ptr += 1;
                }
                (s, o) if s > o => {
                    buffer.push(0);
                    other_ptr += 1;
                }
                (s, _) => {
                    buffer.push(s);
                    self_ptr += 1;
                    other_ptr += 1;
                }
            }
        }
        
        buffer.extend_from_slice(&self.ids[self_ptr..]);
        buffer.extend_from_slice(&other.ids[other_ptr..]);
        
        self.ids = buffer
    }
    
    pub fn contains(&self, id: usize) -> bool {
        self.ids.binary_search(&id).is_ok()
    }
}

impl<'a> Eq for &'a MultiId {}

impl<'a> PartialEq for &'a MultiId {
    fn eq(&self, rhs: &Self) -> bool {
        if self.ids.len() != rhs.ids.len() {
            return false;
        }

        for i in 0..self.ids.len() {
            if self.ids[i] != rhs.ids[i] {
                return false;
            }
        }

        true
    }
}

impl<'a> IntoIterator for &'a MultiId {
    type Item=&'a usize;

    type IntoIter=std::slice::Iter<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.ids.iter()
    }
}

impl Default for MultiId {
    fn default() -> Self {
        MultiId { ids: Vec::new() }
    }
}

impl std::fmt::Display for MultiId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        
        for id in self.ids.iter() {
            f.write_fmt(format_args!("{}, ", id))?;
        }
        
        f.write_str("]")?;
        
        Ok(())
    }
}