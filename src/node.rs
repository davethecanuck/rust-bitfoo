use crate::{Addr,BitVec256};
use std::ops::Index;
use std::iter::Iterator;

#[derive(Debug)]
pub enum Content {
    Bits(Vec<u64>),
    Nodes(Vec<Node>),
}

#[derive(Debug)]
pub struct Node {
    pub level: u8,
    index: BitVec256,  // Set when the given key has 1's
    keys: Vec<u8>,     // List of keys that aligns with bits/child_nodes
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
            level,
            index: BitVec256::new(),
            keys: Vec::with_capacity(1),
            content
        }
    }
    
    // Return the Ok(offset) where this key is found, or
    // the Err(offset) where it should be
    pub fn search(&self, key: u8) -> Result<u8,u8> {
        match self.keys.binary_search(&key) {
            Ok(offset) => Ok(offset as u8),
            Err(offset) => Err(offset as u8)
        }
    }

    // Set the bit corresponding to this address 
    pub fn set(&mut self, addr: &Addr) {
        // Set the index bit
        let key = addr.key(self.level);
        self.index.set(key);
        let offset = self.search(key); // Result(ok(offset), err(offset))

        // Might be nice to split into bit and children functions, 
        // but I'm avoiding the mut borrow checker
        match &mut self.content {
            Content::Bits(vec) => {
                // Do bit level set on u64 bit vector
                let bitmask = 0x1 << addr.key(0);  // Bit offset
                match offset {
                    Ok(off) => {
                        vec[off as usize] |= bitmask;
                    },
                    Err(off) => {
                        // Insert into the bitmask vector and keys 
                        // vector in parallel
                        vec.insert(off as usize, bitmask);
                        self.keys.insert(off as usize, key);
                    }
                }
            },
            Content::Nodes(vec) => {
                match offset {
                    Ok(off) => {
                        let node = &mut vec[off as usize];
                        node.set(addr);
                    },
                    Err(off) => { 
                        // Insert a node into the vector at the 
                        // next level down
                        let mut node = Node::new(self.level - 1);
                        node.set(addr);
                        
                        // Insert into the content vector and keys 
                        // vector in parallel
                        vec.insert(off as usize, node);
                        self.keys.insert(off as usize, key);
                    }
                }
            }
        }
    }
    
    // Clear the bit corresponding to this address 
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
                        }
                    },
                    Err(_off) => {
                        // Do nothing - we're clearing a bit that
                        // wasn't set.
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

    // Return the state of the bit for this address
    pub fn get(&self, addr: &Addr) -> bool {
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
                    // node not found means it's all 1's
                }
            }
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
            level: self.level,
            index: self.index.clone(),
            keys: self.keys.to_vec(),
            content: content
        }
    }
}

// Implement [u64] operator
impl Index<u64> for Node {
    type Output = bool;

    fn index(&self, bitno: u64) -> &Self::Output {
        // Can't easily return self.get() as
        // it is a reference to a local var.
        let addr = Addr::new(bitno);
        match self.get(&addr) {
            true => &true,
            false => &false
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
            true => &true,
            false => &false
        }
    }
}


//==============================================
// Unit tests
//==============================================

#[cfg(test)]
mod tests {
    use crate::{Node,Addr};

    #[test]
    fn index() {
        // Set a few bits, check index, then clear and check
        let mut node = Node::new(1);
        assert_eq!(node.index.raw_data(0), 0b_0000);
        node.set(&Addr::new(0));
        assert_eq!(node.index.raw_data(0), 0b_0001);
        node.set(&Addr::new(5));
        assert_eq!(node.index.raw_data(0), 0b_0001);
        node.clear(&Addr::new(0));
        assert_eq!(node.index.raw_data(0), 0b_0001);
        node.clear(&Addr::new(5));
        assert_eq!(node.index.raw_data(0), 0b_0000);
    }
}

