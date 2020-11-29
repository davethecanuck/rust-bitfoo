use crate::{LeafVec,BranchVec};

#[derive(Debug)]
pub enum ChildVec{
    Branch(Vec<BranchVec>),
    Leaf(Vec<LeafVec>),
    None,
}

impl Clone for ChildVec {
    fn clone(&self) -> ChildVec {
        match self {
            ChildVec::Branch(v) => ChildVec::Branch(v.to_vec()),
            ChildVec::Leaf(v) => ChildVec::Leaf(v.to_vec()),
            ChildVec::None => ChildVec::None,
        }
    }
}
