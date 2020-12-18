use crate::{Addr,BitVec256};

#[derive(Debug)]
pub struct KeyIndex {
    pub level: u8,
    nodes: BitVec256, // Child nodes in the tree
    runs:  BitVec256, // Child nodes that are all 1's (all set)
}

#[derive(Debug)]
pub enum KeyState {
    Run,
    Found(usize),   // offset
    Missing(usize), // offset
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
    
    // Return true if the runs vector is full
    pub fn is_all_runs(&self) -> bool {
        self.runs.is_full()
    }

    pub fn set_all_runs(&mut self) {
        self.runs.set_all();
    }
    
    // Check if this address is in our index
    // EYE - offset returned ought to be usize as
    // that is always how it is used to access Vec data
    pub fn search(&self, addr: &Addr) -> KeyState {
        let key = self.key(addr);
        if self.runs.get(key) {
            return KeyState::Run;
        }
        else {
            match self.nodes.offset(key) {
                Ok(offset) => KeyState::Found(offset as usize),
                Err(offset) => KeyState::Missing(offset as usize),
            }
        }
    }

    // Return the nodes offset corresponding to this Addr
    // (missing or not). 
    pub fn offset(&self, addr: &Addr) -> usize {
        let key = self.key(addr);
        match self.nodes.offset(key) {
            Ok(offset) => offset as usize,
            Err(offset) => offset as usize,
        }
    }

    // Return the key value for this Addr
    fn key(&self, addr: &Addr) -> u8 {
        addr.key(self.level)
    }

    // Mark this key as 'all set' (all node
    // bits are set)
    pub fn run(&mut self, addr: &Addr) {
        let key = self.key(addr);
        self.nodes.clear(key);
        self.runs.set(key);
    }

    // Mark this key as having a bit set
    pub fn set(&mut self, addr: &Addr) {
        let key = self.key(addr);
        self.nodes.set(key);
        self.runs.clear(key);
    }

    // Remove this key from the index
    pub fn clear(&mut self, addr: &Addr) {
        let key = self.key(addr);
        self.nodes.clear(key);
        self.runs.clear(key);
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
#[path = "./tests/key_test.rs"]
mod tests;
