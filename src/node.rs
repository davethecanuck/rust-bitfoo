use crate::{Addr,KeyIndex,KeyState,KeyIndexIterator};
use std::ops::Index;
use std::iter::Iterator;

#[derive(Debug)]
pub enum Content {
    Bits(Vec<u64>),
    Nodes(Vec<Node>),
}

#[derive(Debug)]
pub struct Node {
    pub index: KeyIndex,   // Indexes content keys by vec offset
    content: Content,      // Contains vec of either u64 bits or Nodes
}
    
// Public interface
impl Node {
    // Constructor
    pub fn new(level: u8) -> Self {
        // Level 1 contains a list of 64-bit (2^6) chunks (which are level 0)
        // Level 2+ contains up to 256 (2^8) child nodes
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

    // Return Our level
    pub fn level(&self) -> u8 {
        self.index.level
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
            KeyState::Run(_key) => true,
            KeyState::Node(_key, offset) => {
                match &self.content {
                    Content::Bits(vec) => {
                        vec[offset] & 0x1 << addr.key(0) > 0
                    },
                    Content::Nodes(vec) => {
                        vec[offset].get(addr)
                    }
                }
            },
            KeyState::Missing(_key, _offset) => {
                false
            }
        }
    }

    // Add the given node as a child
    pub fn add_node(&mut self, node: Node) {
        match &mut self.content {
            Content::Bits(_vec) => {
                // Someone is mis-using interface
                panic!("Cannot call add_node on level 1 node");
            },
            Content::Nodes(vec) => {
                // NOTE: Should only be used to append
                // the first node (called by BitFooVec)
                vec.push(node);
                self.index.set_node_bit(0);
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
            KeyState::Run(_key) => (),  
            KeyState::Node(_key, offset) => {
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
            KeyState::Missing(_key, offset) => {
                // Just set - no run possible
                vec.insert(offset, 0x1 << addr.key(0));
                index.set(addr);
            },
        }
    }

    // Set a bit for a 'Nodes' type content
    fn set_nodes(index: &mut KeyIndex, vec: &mut Vec<Node>, addr: &Addr) {
        match index.search(addr) {
            KeyState::Run(_key) => (),    // No-op to set on a run
            KeyState::Node(_key, offset) => {
                // Tell child node to set bit
                vec[offset].set(addr);
                if vec[offset].index.is_all_runs() {
                    // Run detected - remove node and update index
                    vec.remove(offset);
                    index.run(addr);
                }
            },
            KeyState::Missing(_key, offset) => {
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
            KeyState::Run(_key) => {
                // It's not longer a run, so need to add a u64 to our 
                // content vector with all bits set but the cleared bit.
                // This will be the only element in the vector (offset=0)
                let bitmask = !(0x1 << addr.key(0));  
                vec.push(bitmask);
                index.set(addr); 
            },
            KeyState::Node(_key, offset) => {
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
            KeyState::Missing(_key, _offset) => (), // No-op to clear all 0's
        }
    }
    
    // Clear a bit for a 'Nodes' type content
    fn clear_nodes(index: &mut KeyIndex, vec: &mut Vec<Node>, addr: &Addr) {
        match index.search(addr) {
            KeyState::Run(_key) => {
                // Insert a node with 'all runs' index, then
                // clear this Addr
                let mut node = Node::new(index.level - 1);
                node.index.set_all_runs();
                node.clear(addr);

                // Add to our vector and set the index (offset 0)
                vec.push(node); 
                index.set(addr);  
            },
            KeyState::Node(_key, offset) => {
                vec[offset].clear(addr);
                index.clear(addr);
            },
            KeyState::Missing(_key, _offset) => (), // No-op if all 0's
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

// Iterator for bit numbers is given a starting Addr
impl Node {
    pub fn iter(&self, addr: Addr) -> NodeIterator {
        let mut index_iter = self.index.iter();
        let key_state = index_iter.next();

        NodeIterator {
            node: self,
            index_iter, 
            key_state, 
            addr: addr, // Includes keys from higher levels
        }
    }
}

pub struct NodeIterator<'a> {
    node: &'a Node,
    index_iter: KeyIndexIterator<'a>,
    key_state: Option<KeyState>,
    addr: Addr,
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = Addr;
    // EYE - Should pass an Addr around, but return
    // u64 (bitno). This simplifies iterating through
    // runs

    fn next(&mut self) -> Option<Self::Item> {
        // EYE - Need Addr methods to show the range
        // of values for a given Addr prefix and level
        // - do we need different types of iterators?
        let mut result = None;

        match self.key_state {
            Some(KeyState::Node(key, offset)) => {
                match &self.node.content {
                    Content::Nodes(_vec) => {
                        // Get iterator for the child
                        let mut child_addr = self.addr.clone();
                        child_addr.set(self.node.level() - 1, key);
                        // EYE - how do we get back here...
                        // - need to map this out
                        // - Can't have iterator containing iterator
                        // as size is undefined (unless we box)
                        // 
                        // EYE - Use a NodeContentIterator enum which 
                        // has iterators for Run, Bits, or Nodes
                        // - Consider putting modules into src/node/node.rs
                        //   and src/node/node_test.rs, .../node_iter.rs, etc
                        // - This gives us private node::iter and node::test submodules
                    },
                    Content::Bits(_vec) => {
                        // EYE - basically need a Bits iterator...
                    },
                }

                // EYE need to iterate on the child
            },
            Some(KeyState::Run(key)) => {
                // Use Addr method to get start and
                // end bitno. Keep this in the iterator
                // and iterate over the bits.
            },
            _ => ()
        }
        result
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

#[cfg(test)]
#[path = "./tests/node_test.rs"]
mod tests;
