use crate::Addr;

// Temporary object used to hold elements taken from
// or sent to the LeafNodeVec
#[derive(Debug)]
pub struct LeafNode {
    offset: usize,
    key: u8,
    bitmask: u64,
}

// Leaf nodes stored as parallel vectors of 
// keys and bits 
#[derive(Debug)]
pub struct LeafNodeVec {
    keys: Vec<u8>,
    bits: Vec<u64>,
}

impl LeafNodeVec {
    // Constructor
    pub fn new() -> Self {
        LeafNodeVec {
            keys: Vec::with_capacity(1),
            bits: Vec::with_capacity(1),
        }
    }

    // Set the bit corresponding to this address 
    pub fn set(&mut self, addr: &Addr) {
        // Level is always 1 for leaf nodes
        let key = addr.key(1);
        let bitmask = 0x1 << addr.key(0);

        match self.search(key) {
            Ok(node) => {
                // Found the node, so set the bit
                self.bits[node.offset] |= bitmask;
            },
            Err(offset) => {
                // Create the node and insert 
                let node = LeafNode{ offset, key, bitmask };
                self.insert(node);
            }
        }
    }

    // Return the state of the bit corresponding to
    // this address
    pub fn get(&mut self, addr: &Addr) -> bool {
        // Level is always 1 for leaf nodes
        let key = addr.key(1);
        let bitmask = 0x1 << addr.key(0);

        match self.search(key) {
            Ok(node) => {
                self.bits[node.offset] & bitmask > 0
            },
            Err(_offset) => {
                false
            }
        }
    }


    // Add this node data onto the vec
    pub fn insert(&mut self, node: LeafNode) {
        self.keys.insert(node.offset, node.key);
        self.bits.insert(node.offset, node.bitmask);
    }
    
    // Create LeafNode temporary object representing 
    // the key and bits at an offset. If no data at 
    // that offset, return error with the next 
    // available offset
    pub fn node(&self, offset: usize) -> Result<LeafNode,usize> {
        let last_offset = self.keys.len();
        // EYE - max of 255

        if offset < last_offset {
            let node = LeafNode {
                offset,
                key: self.keys[offset], 
                bitmask: self.bits[offset]
            };
            Ok(node)
        }
        else {
            Err(last_offset)
        }
    }

    // Return the bits data which matches this key.
    // If not found, return error with the offset
    // where it would need to be located.
    pub fn search(&self, key: u8) -> Result<LeafNode,usize> {
        match self.keys.binary_search(&key) {
            Ok(offset) => self.node(offset),
            Err(offset) => Err(offset)
        }
    }
}

impl Clone for LeafNodeVec {
    fn clone(&self) -> LeafNodeVec {
        LeafNodeVec { 
            keys: self.keys.to_vec(),
            bits: self.bits.to_vec(),
        }
    }
}
