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

    pub fn set(&mut self, bitno: u64) {
        let addr = Addr::new(bitno);
        println!("--- Setting bit {} ---", bitno);
        println!("Node level is {}", addr.level());
        for level in (0..=addr.level()).rev() {
            println!("    {} => {}", level, addr.key(level));
        }
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
