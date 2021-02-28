use std::fmt;
use crate::{Addr,KeyIndex,KeyState};
use crate::node::iter::NodeIterator;

pub enum Content {
    Bits(Vec<u64>),
    Nodes(Vec<Node>),
}

// Debug interface for Content
impl fmt::Debug for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Content::Bits(v) => {
                // Write first and last few elements of vector in hex
                write!(f, "Bits[len={}]:[ ", v.len())?;
                for (i, element) in v.iter().enumerate().rev() {
                    if i < 3 || i >= (v.len() - 3) {
                        write!(f, "{}:[{:#b}] ", i, element)?;
                    }
                    else if i == 3 {
                        write!(f, "{}", ".....".to_string())?;
                    }
                }
                write!(f, "{}", "]".to_string())
            },
            Content::Nodes(v) => {
                // Write first and last few elements of vector in hex
                write!(f, "Nodes[len={}]:[ ", v.len())?;
                for (i, element) in v.iter().enumerate().rev() {
                    if i < 3 || i > (v.len() - 3) {
                        write!(f, "{}:[{:?}] ", i, element)?;
                    }
                    else if i == 3 {
                        write!(f, "{}", ".....".to_string())?;
                    }
                }
                write!(f, "{}", "]".to_string())
            }
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub index: KeyIndex,          // Indexes content keys by vec offset
    pub (super) content: Content, // Contains vec of either u64 bits or Nodes
}

// Public interface
impl Node {
    // Constructor
    pub fn new(level: u8) -> Self {
        // Level 1 contains a list of 64-bit (2^6) chunks (which are level 0)
        // Level 2+ contains up to 256 (2^8) child nodes
        let content = match level {
            1 => Content::Bits(Vec::with_capacity(1)),
            2..=9 => Content::Nodes(Vec::with_capacity(1)),
            _ => {
                panic!("Nodes can only be constructed with levels 1-9");
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

    // Bitno iterator starting from a node
    pub fn iter(&self) -> NodeIterator {
        NodeIterator::new(self, Addr::new(0))
    }
}

// Private helper functions.
// NOTE: No &self passed in as we want to avoid obtaining
// a second mutable borrow on &self. Instead we are passing in the
// structure elements as mutable references
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