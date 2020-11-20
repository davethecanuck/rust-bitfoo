use crate::Node;

// Using a Vec wrapper so we can enforce max size
// of 2^8 and implement custom resizing algo

#[derive(Debug)]
pub struct NodeVec {
    vec: Vec<Node>,
}

impl NodeVec {
    pub fn new() -> Self {
        NodeVec {
            vec: Vec::with_capacity(1)
        }
    }

    pub fn add(&mut self, node: Node) {
        // EYE - check capacity
        self.vec.push(node);
    }

    pub fn insert(&mut self, node: Node, offset: u8) {
        // EYE - check capacity
        self.vec.insert(offset as usize, node);
    }
}

impl Clone for NodeVec {
    fn clone(&self) -> NodeVec {
        NodeVec { 
            vec: self.vec.to_vec()
        }
    }
}
