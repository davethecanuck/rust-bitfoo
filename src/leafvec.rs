use crate::{Addr,BitVec256};

// Temporary object used to hold elements taken from
// or sent to the LeafVec
#[derive(Debug)]
pub struct LeafNode {
    offset: u8,
    key: u8,
    bitmask: u64,
}

// Leaf nodes stored as parallel vectors of 
// keys and bits 
#[derive(Debug)]
pub struct LeafVec {
    keys: Vec<u8>,
    bits: Vec<u64>,
    child_map: BitVec256,  
    
    // NOTE: If we have a child set but no 
    // corresponding 'bits' data, it's assumed
    // to be all ones (by key)
}

impl LeafVec {
    // Constructor
    pub fn new() -> Self {
        LeafVec {
            keys: Vec::with_capacity(1),
            bits: Vec::with_capacity(1),
            child_map: BitVec256::new(),
        }
    }

    // Set the bit corresponding to this address 
    pub fn set(&mut self, addr: &Addr) {
        // Level is always 1 for leaf nodes
        let key = addr.key(1);
        self.child_map.set(key);

        let bitmask = 0x1 << addr.key(0);
        match self.search(key) {
            Ok(node) => {
                // Found the node, so set the bit
                self.bits[node.offset as usize] |= bitmask;
            },
            Err(offset) => {
                // Create the node and insert 
                let node = LeafNode{ 
                    offset, 
                    key, 
                    bitmask, 
                };
                self.insert(node);
            }
        }
    }

    // Return the state of the bit corresponding to
    // this address
    pub fn get(&mut self, addr: &Addr) -> bool {
        // Level is always 1 for leaf nodes
        let key = addr.key(1);
        if !self.child_map[key] {
            return false;
        }

        let bitmask = 0x1 << addr.key(0);
        match self.search(key) {
            Ok(node) => {
                self.bits[node.offset as usize] & bitmask > 0
            },
            Err(_offset) => {
                // May be true if child_map is set for this key
                // (indicates all 1s at this offset)
                self.child_map[key] 
            }
        }
    }


    // Add this node data onto the vec
    pub fn insert(&mut self, node: LeafNode) {
        self.keys.insert(node.offset as usize, node.key);
        self.bits.insert(node.offset as usize, node.bitmask);

        if node.bitmask == 0 {
            self.child_map.clear(node.key);
        }
        else {
            self.child_map.set(node.key);
        }
    }
    
    // Create LeafNode temporary object representing 
    // the key and bits at an offset. If no data at 
    // that offset, return error with the next 
    // available offset
    pub fn node(&self, offset: u8) -> Result<LeafNode,u8> {
        let last_offset = self.keys.len();

        if (offset as usize) < last_offset {
            let node = LeafNode {
                offset,
                key: self.keys[offset as usize], 
                bitmask: self.bits[offset as usize],
            };
            Ok(node)
        }
        else {
            Err(last_offset as u8)
        }
    }

    // Return the bits data which matches this key.
    // If not found, return error with the offset
    // where it would need to be located.
    pub fn search(&self, key: u8) -> Result<LeafNode,u8> {
        match self.keys.binary_search(&key) {
            Ok(offset) => self.node(offset as u8),
            Err(offset) => Err(offset as u8)
        }
    }
}

impl Clone for LeafVec {
    fn clone(&self) -> LeafVec {
        LeafVec { 
            keys: self.keys.to_vec(),
            bits: self.bits.to_vec(),
            child_map: self.child_map.clone(),
        }
    }
}
