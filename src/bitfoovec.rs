use crate::{Node,Addr};

// Main container for the bit vector
#[derive(Debug)]
pub struct BitFooVec {
    len: u64,   // Set to the max bitno
    level: u8,  // Must be same level as max bit level
    root_node: Node,
}

// Public interface
impl BitFooVec {
    pub fn new() -> Self {
        BitFooVec {
            len: 0, 
            level: 1,
            root_node: Node::new(1),  // Can this be level 9?
        }
    }

    // We delegate most logic to the Node but need
    // to insert parent nodes if the level of the Addr
    // is too high
    pub fn set(&mut self, bitno: u64) {
        let addr = Addr::new(bitno);
        while addr.level > self.level {
            // Replace root with new one at next level up
            self.level += 1;
            let old_root = std::mem::replace(&mut self.root_node, 
                                                 Node::new(self.level));

            // Set old_root to be child of new root 
            self.root_node.add_node(old_root); 
            self.root_node.index.set(&addr); // EYE test
        }

        // len attribute is the last set bitno
        if bitno >= self.len {
            self.len = bitno+1;
        }
    }

    pub fn clear(&mut self, bitno: u64) {
        // Don't need to insert nodes to represent a high bit
        // 0 - it's already implied to be 0
        let addr = Addr::new(bitno);
        if addr.level <= self.level {
            self.root_node.clear(&addr);
        }
    }
}

impl Clone for BitFooVec {
    fn clone(&self) -> BitFooVec {
        BitFooVec { 
            len: self.len.clone(),
            level: self.level.clone(),
            root_node: self.root_node.clone(),
        }
    }
}
