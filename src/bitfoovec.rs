use crate::{Node,Addr};

// Main container for the bit vector
#[derive(Debug)]
pub struct BitFooVec {
    root_node: Node,
}

// Public interface
impl BitFooVec {
    pub fn new() -> Self {
        BitFooVec {
            root_node: Node::new(1),  // EYE - Can this be level 9?
        }
    }

    // Return the level from the root node
    pub fn level(&self) -> u8 {
        self.root_node.level()
    }

    // We delegate most logic to the Node but need
    // to insert parent nodes if the level of the Addr
    // is too high
    pub fn set(&mut self, bitno: u64) {
        let addr = Addr::new(bitno);
        let mut level = self.level();

        while addr.node_level > level {
            // Replace root with new one at next level up
            level += 1;
            let old_root = std::mem::replace(
                &mut self.root_node, Node::new(level));

            // Set old_root to be child of new root 
            self.root_node.add_node(old_root); 
        }
        self.root_node.set(&addr);
    }

    // Return state of this bit
    pub fn get(&self, bitno: u64) -> bool {
        let addr = Addr::new(bitno);
        if addr.node_level > self.level() {
            false
        }
        else {
            self.root_node.get(&addr)
        }
    }

    pub fn clear(&mut self, bitno: u64) {
        // Don't need to insert nodes to represent a high bit
        // 0 - it's already implied to be 0
        let addr = Addr::new(bitno);
        if addr.node_level <= self.level() {
            self.root_node.clear(&addr);
        }
    }
}

impl Clone for BitFooVec {
    fn clone(&self) -> BitFooVec {
        BitFooVec { 
            root_node: self.root_node.clone(),
        }
    }
}

#[cfg(test)]
#[path = "./tests/bitfoovec_test.rs"]
mod tests;