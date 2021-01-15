use crate::{Addr,BitVec256,BitVec256Iterator};
use std::iter::Iterator;

#[derive(Debug)]
pub struct KeyIndex {
    pub level: u8,
    pub nodes: BitVec256, // Child nodes in the tree
    pub runs:  BitVec256, // Child nodes that are all 1's (all set)
}

#[derive(Debug)]
pub enum KeyState {
    Run(u8),            // key
    Node(u8, usize),    // key, offset
    Missing(u8, usize), // key, offset
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

    // Marks runs vector as all set
    pub fn set_all_runs(&mut self) {
        self.runs.set_all();
    }

    // Return true if nodes vector is all 0s
    pub fn is_nodes_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    // Return true if nodes vector is all 1s
    pub fn is_nodes_full(&self) -> bool {
        self.nodes.is_full()
    }

    // Return true if runs vector is all 0s
    pub fn is_runs_empty(&self) -> bool {
        self.runs.is_empty()
    }

    // Return true if runs vector is all 0s
    pub fn is_runs_full(&self) -> bool {
        self.runs.is_full()
    }

    // Return true if this Addr is set for our node index
    pub fn is_node(&self, addr: &Addr) -> bool {
        let key = self.key(addr);
        self.nodes.get(key)
    }

    // Return true if this Addr is set for our run index
    pub fn is_run(&self, addr: &Addr) -> bool {
        let key = self.key(addr);
        self.runs.get(key)
    }

    // Check if this address is in our index, returning
    // the appropriate KeyState instance (Run, Node, Missing)
    pub fn search(&self, addr: &Addr) -> KeyState {
        let key = self.key(addr);
        if self.runs.get(key) {
            return KeyState::Run(key);
        }
        else {
            match self.nodes.offset(key) {
                Ok(offset) => KeyState::Node(key, offset as usize),
                Err(offset) => KeyState::Missing(key, offset as usize),
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
    pub fn key(&self, addr: &Addr) -> u8 {
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

    // Set a node bit directly. Used when BitFooVec
    // is adding a child node directly
    pub fn set_node_bit(&mut self, bitno: u8) {
        self.nodes.set(bitno);
    }

    // Return an iterator
    pub fn iter(&self) -> KeyIndexIterator {
        let mut node_iter = self.nodes.iter();
        let mut run_iter = self.runs.iter();

        KeyIndexIterator {
            node_key: node_iter.next(),
            node_offset: 0,
            node_iter: node_iter,
            run_key: run_iter.next(),
            run_iter: run_iter,
        }
    }
}

// Iterator over index returns the sequence
// of KeyState's
pub struct KeyIndexIterator<'a> {
    node_iter: BitVec256Iterator<'a>,
    node_key: Option<u8>,
    node_offset: usize,
    run_iter: BitVec256Iterator<'a>,
    run_key: Option<u8>,
}

impl<'a> Iterator for KeyIndexIterator<'a> {
    type Item = KeyState;

    fn next(&mut self) -> Option<KeyState> {
        let mut result = None;

        match (self.node_key, self.run_key) {
            (Some(node_key), Some(run_key)) => {
                // Return the next of the node or run keys
                if node_key < run_key {
                    result = Some(KeyState::Node(node_key, self.node_offset));
                    self.node_key = self.node_iter.next();
                    self.node_offset += 1;
                }
                else {
                    result = Some(KeyState::Run(run_key));
                    self.run_key = self.run_iter.next();
                }
            },
            (Some(node_key), None) => {
                // Only a node key found
                result = Some(KeyState::Node(node_key, self.node_offset));
                self.node_key = self.node_iter.next();
                self.node_offset += 1;
            },
            (None, Some(run_key)) => {
                // Only a run key found
                result = Some(KeyState::Run(run_key));
                self.run_key = self.run_iter.next();
            }
            _ => ()
        }

        // EYE - convert to KeyState and need to increment the
        // offset in the iterator
        result
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
