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
                match self.index.search(addr) {
                    KeyState::Run => (),  
                    KeyState::Found(offset) => {
                        // Update existing bitmask
                        let idx = offset as usize;
                        let newbits = vec[idx] | 0x1 << addr.key(0);

                        if newbits == u64::MAX {
                            // Run detected - remove node and update index
                            vec.remove(idx);
                            self.index.run(addr);
                        }
                        else {
                            // Just save the bits
                            vec[idx] = newbits;
                        }
                    },
                    KeyState::Missing(offset) => {
                        // Just set - no run possible
                        let idx = offset as usize;
                        vec.insert(idx, 0x1 << addr.key(0));
                        self.index.set(addr);
                    },
                }
            },
            Content::Nodes(vec) => {
                match self.index.search(addr) {
                    KeyState::Run => (), 
                    KeyState::Found(offset) => {
                        // Tell child node to set bit
                        let idx = offset as usize;
                        vec[idx].set(addr);

                        if vec[idx].index.is_all_runs() {
                            // Run detected - remove node and update index
                            vec.remove(idx);
                            self.index.run(addr);
                        }
                    },
                    KeyState::Missing(offset) => {
                        // No run possible on first insert
                        let idx = offset as usize;
                        let mut node = Node::new(self.index.level - 1);
                        node.set(addr);
                        vec.insert(idx, node);
                        self.index.set(addr);
                    },
                }
            }
        }
    }

    // Clear the bit corresponding to this address 
    /*
    pub fn clear(&mut self, addr: &Addr) {
        // clear the index bit
        let key = addr.key(self.level);
        let offset = self.search(key); // Result(ok(offset), err(offset))

        match &mut self.content {
            Content::Bits(vec) => {
                // Do bit level set on u64 bit vector
                let bitmask = !(0x1 << addr.key(0));  // Bit offset
                match offset {
                    Ok(off) => {
                        vec[off as usize] &= bitmask;
                        if vec[off as usize] == 0 {
                            // Bitmask has no bits set, so clear index
                            self.index.clear(key);
                            //EYE - should remove node+key 
                        }
                    },
                    Err(_off) => {
                        // Do nothing - we're clearing a bit that
                        // wasn't set.
                        // EYE - but need to check bitvec256 index
                    }
                }
            },
            Content::Nodes(vec) => {
                match offset {
                    Ok(off) => {
                        let node = &mut vec[off as usize];
                        node.clear(addr);
                        if node.index.is_empty() {
                            self.index.clear(key);
                            //EYE - should remove node+key 
                        }
                    },
                    Err(_off) => { 
                        // Do nothing - we're clearing a bit that
                        // wasn't set.
                    }
                }
            }
        }
    }
*/

    // Return the state of the bit for this address
    pub fn get(&self, addr: &Addr) -> bool {
        false
    }
/*
        let key = addr.key(self.level);
        if !self.index[key] {
            // Shortcut if index not set
            return false;
        }

        // Check our content vector
        let offset = self.search(key);
        match &self.content {
            Content::Bits(vec) => {
                let bitmask = 0x1 << addr.key(0);  // Bit offset
                match offset {
                    Ok(off) => vec[off as usize] & bitmask > 0,
                    Err(_off) => true,
                    // bitmask not found means it's all 1's
                }
            },
            Content::Nodes(vec) => {
                match offset {
                    Ok(off) => {
                        let node = &vec[off as usize];
                        node.get(addr)
                    },
                    Err(_off) => true,
                    // Index is set but node not found
                    // means it's all 1's
                }
            }
        }
    }
    */
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
