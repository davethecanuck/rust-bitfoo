use crate::{Addr,BitVec256};

#[derive(Debug)]
pub struct KeyIndex {
    pub level: u8,
    nodes: BitVec256, // Child nodes in the tree
    runs:  BitVec256, // Child nodes that are all 1's (all set)
}

#[derive(Debug)]
pub enum KeyState {
    AllSet,
    Found(u8, u8),   // key,  offset
    Missing(u8, u8), // key,  offset
}

// Public interface
impl KeyIndex {
    // Constructor
    pub fn new(level: u8) -> Self {
        KeyIndex {
            level,
            nodes: BitVec256::new(),
            runs: BitVec256::new(),
        }
    }
    
    // Check if this address is in our index
    pub fn search(&self, addr: &Addr) -> KeyState {
        let key = self.key(addr);
        if self.runs.get(key) {
            return KeyState::AllSet;
        }
        else {
            match self.nodes.offset(key) {
                Ok(offset) => KeyState::Found(key, offset),
                Err(offset) => KeyState::Missing(key, offset),
            }
        }
    }

    // Return the key value for this Addr
    fn key(&self, addr: &Addr) -> u8 {
        addr.key(self.level)
    }

    // Mark this key as 'all set' (all node
    // bits are set)
    pub fn mark_run(&mut self, key: u8, offset: u8) {
        self.nodes.clear(key);
        self.runs.set(key);
    }

    // Mark this key as having a bit set
    pub fn insert(&mut self, key: u8, offset: u8) {
        self.nodes.set(key);
    }

    // Remove this key from the index
    pub fn remove(&mut self, key: u8, offset: u8) {
        self.nodes.clear(key);
        // EYE - if all set then need to set all nodes
        // except this one
    }
}

// Clone interface
impl Clone for KeyIndex {
    fn clone(&self) -> KeyIndex {
        KeyIndex { 
            level: self.level,
            nodes: self.nodes.clone(),
            runs: self.runs.clone(),
        }
    }
}

#[cfg(test)]
#[path = "./key_test.rs"]
mod tests;
