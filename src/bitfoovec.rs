use crate::{ChildVec,BitVec256};

// Main container for the bit vector
#[derive(Debug)]
pub struct BitFooVec {
    len: u64,
    level: u8,
    children: ChildVec,   
    child_map: BitVec256,
}

impl BitFooVec {
    pub fn new() -> Self {
        BitFooVec {
            len: 0, 
            level: 1,
            children: ChildVec::None,
            child_map: BitVec256::new(),
        }
    }
}

impl Clone for BitFooVec {
    fn clone(&self) -> BitFooVec {
        BitFooVec { 
            len: self.len.clone(),
            level: self.level.clone(),
            children: self.children.clone(),
            child_map: self.child_map.clone(),
        }
    }
}
