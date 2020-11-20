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
