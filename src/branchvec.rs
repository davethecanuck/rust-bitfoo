use crate::{BitVec256,ChildVec};

#[derive(Debug)]
pub struct BranchVec {
    keys: Vec<u8>,
    level: u8,
    children: ChildVec,
    child_map: BitVec256,
}

impl BranchVec {
    // Constructor
    pub fn new(level: u8) -> Self {
        BranchVec {
            keys: Vec::with_capacity(1),
            level: level,
            children: ChildVec::None,
            child_map: BitVec256::new(),
        }
    }
}

impl Clone for BranchVec {
    fn clone(&self) -> BranchVec {
        BranchVec { 
            keys: self.keys.to_vec(),
            level: self.level,
            children: self.children.clone(),
            child_map: self.child_map.clone(),
        }
    }
}
