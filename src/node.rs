use crate::{Addr,KeyIndex,KeyState};
use std::ops::Index;
//use std::iter::Iterator;

#[derive(Debug)]
pub enum Content {
    Bits(Vec<u64>),
    Nodes(Vec<Node>),
}

#[derive(Debug)]
pub struct Node {
    index: KeyIndex,   // Indexes content keys by vec offset
    content: Content,  // Contains vec of either u64 bits or Nodes
}

// Public interface
impl Node {
    // Constructor
    pub fn new(level: u8) -> Self {
        // Level 1 contains a list of 64-bit chunks (which are level 0)
        // Level 2+ contains child nodes
        let content = match level {
            1 => Content::Bits(Vec::with_capacity(1)),
            2..=8 => Content::Nodes(Vec::with_capacity(1)),
            _ => {
                panic!("Nodes can only be constructed with levels 1-8");
            }
        };
                
        // Contains child nodes 
        Node {
            index: KeyIndex::new(level),
            content
        }
    }

    // Set the bit corresponding to this address.
    pub fn set(&mut self, addr: &Addr) {
        match &mut self.content {
            Content::Bits(vec) => {
                Self::set_bits(&mut self.index, vec, addr);
            },
            Content::Nodes(vec) => {
                Self::set_nodes(&mut self.index, vec, addr);
            }
        }
    }

    // Clear the bit corresponding to this address 
    pub fn clear(&mut self, addr: &Addr) {
        match &mut self.content {
            Content::Bits(vec) => {
                Self::clear_bits(&mut self.index, vec, addr);
            },
            Content::Nodes(vec) => {
                Self::clear_nodes(&mut self.index, vec, addr);
            }
        }
    }

    // Return the state of the bit for this address
    pub fn get(&self, addr: &Addr) -> bool {
        match self.index.search(addr) {
            KeyState::Run => true,
            KeyState::Found(offset) => {
                match &self.content {
                    Content::Bits(vec) => {
                        vec[offset] & 0x1 << addr.key(0) > 0
                    },
                    Content::Nodes(vec) => {
                        vec[offset].get(addr)
                    }
                }
            },
            KeyState::Missing(_offset) => {
                false
            }
        }
    }
}

// Private helper functions.
// NOTE: No &self passed in as we want to avoid obtaining
// a second mutable borrow on &self. Instead we are passing in the
// structure elements as mustable references
impl Node {
    // Set a bit for a 'Bits' type content
    fn set_bits(index: &mut KeyIndex, vec: &mut Vec<u64>, addr: &Addr) {
        match index.search(addr) {
            KeyState::Run => (),  
            KeyState::Found(offset) => {
                // Update existing bitmask
                let newbits = vec[offset] | 0x1 << addr.key(0);
                if newbits == u64::MAX {
                    // Run detected - remove node and update index
                    vec.remove(offset);
                    index.run(addr);
                }
                else {
                    // Just save the bits
                    vec[offset] = newbits;
                }
            },
            KeyState::Missing(offset) => {
                // Just set - no run possible
                vec.insert(offset, 0x1 << addr.key(0));
                index.set(addr);
            },
        }
    }

    // Set a bit for a 'Nodes' type content
    fn set_nodes(index: &mut KeyIndex, vec: &mut Vec<Node>, addr: &Addr) {
        match index.search(addr) {
            KeyState::Run => (),    // No-op to set on a run
            KeyState::Found(offset) => {
                // Tell child node to set bit
                vec[offset].set(addr);
                if vec[offset].index.is_all_runs() {
                    // Run detected - remove node and update index
                    vec.remove(offset);
                    index.run(addr);
                }
            },
            KeyState::Missing(offset) => {
                // Create the new child node
                let mut node = Node::new(index.level - 1);
                node.set(addr);
                vec.insert(offset, node);
                index.set(addr);
            },
        }
    }
    
    // Clear a bit for a 'Bits' type content
    fn clear_bits(index: &mut KeyIndex, vec: &mut Vec<u64>, addr: &Addr) {
        match index.search(addr) {
            KeyState::Run => {
                // It's not longer a run, so need to add a u64 to our 
                // content vector with all bits set but the cleared bit.
                // This will be the only element in the vector (offset=0)
                let bitmask = !(0x1 << addr.key(0));  
                vec.push(bitmask);
                index.clear(addr); 
            },
            KeyState::Found(offset) => {
                // Update existing bitmask
                let bitmask = !(0x1 << addr.key(0));  
                let newbits = vec[offset] & bitmask;

                if newbits == 0 {
                    // Node is all 0's, so remove
                    vec.remove(offset);
                    index.clear(addr);
                }
                else {
                    // Just save the bits
                    vec[offset] = newbits;
                }
            },
            KeyState::Missing(_offset) => (), // No-op to clear all 0's
        }
    }
    
    // Clear a bit for a 'Nodes' type content
    fn clear_nodes(index: &mut KeyIndex, vec: &mut Vec<Node>, addr: &Addr) {
        match index.search(addr) {
            KeyState::Run => {
                // Insert a node as 'all runs' and then clear 
                // the bit for this addr
                let mut node = Node::new(index.level - 1);
                node.index.set_all_runs();
                node.clear(addr);

                // Add to our vector and set the index (offset 0)
                vec.push(node); 
                index.set(addr);  
            },
            KeyState::Found(offset) => {
                vec[offset].clear(addr);
                index.clear(addr);
            },
            KeyState::Missing(_offset) => (), // No-op if all 0's
        }
    }
}

// Clone interface
impl Clone for Node {
    fn clone(&self) -> Node {
        let content = match &self.content {
            Content::Bits(v) => Content::Bits(v.to_vec()),
            Content::Nodes(v) => Content::Nodes(v.to_vec()),
        };

        Node { 
            index: self.index.clone(),
            content: content
        }
    }
}

// Static references for [] return values
static TRUE: bool = true;
static FALSE: bool = false;

// Implement [u64] operator
impl Index<u64> for Node {
    type Output = bool;

    fn index(&self, bitno: u64) -> &Self::Output {
        // Can't easily return self.get() as
        // it is a reference to a local var.
        let addr = Addr::new(bitno);
        match self.get(&addr) {
            true => &TRUE,
            false => &FALSE
        }
    }
}

// Implement [&Addr] operator
impl Index<&Addr> for Node {
    type Output = bool;

    fn index(&self, addr: &Addr) -> &Self::Output {
        // Can't easily return self.get() as
        // it is a reference to a local var.
        match self.get(addr) {
            true => &TRUE,
            false => &FALSE
        }
    }
}

/* EYE TBD
#[cfg(test)]
#[path = "./tests/node_test.rs"]
mod tests;
*/
