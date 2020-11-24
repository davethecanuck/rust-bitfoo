use crate::{NodeVec,Addr};

// Main container for the bit vector
#[derive(Debug)]
pub struct BitFooVec {
    pub len: u64,
    pub level: u8,
    pub nodes: NodeVec,   
}

impl BitFooVec {
    pub fn new() -> Self {
        BitFooVec {
            len: 0, 
            level: 0,
            nodes: NodeVec::new(),
        }
    }

    // When setting a bit, search for the required
    // node key at the highest level, inserting node
    // if needed. Recursively do for each key level
    // before finally setting the bit. Convert level
    // 0 node to Bits if needed.
    pub fn set(&mut self, bitno: u64) {
        let addr = Addr::new(bitno);
        println!("TBD set bitno={}, addr={:?}", bitno, addr);
/*
        let mut curr_nodevec = self.nodes;
        let mut curr_node = curr_nodevec.get(0);

        // Walk down from top to the bit level, inserting 
        // nodes as needed
        for level in (0..=addr.level()).rev() {
            if level > self.level {
                // We need to point vector at this level
                self.nodes = NodeVec::new();
                self.len = bitno;
                self.level = level;
                self.nodes = top;
                curr_nodevec = self.nodes;
            }

            // Add node to vec if needed
            let key = addr.key(level);
            match curr_nodevec.search(key) {
                Some(offset, node) => {

                },
                None(offset) => {
                }
            }

            println!("    {} => {}", level, addr.key(level));
        }
*/
    }
}

impl Clone for BitFooVec {
    fn clone(&self) -> BitFooVec {
        BitFooVec { 
            len: self.len.clone(),
            level: self.level.clone(),
            nodes: self.nodes.clone(),
        }
    }
}
