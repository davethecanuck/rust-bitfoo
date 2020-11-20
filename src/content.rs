use crate::{Node,NodeVec};

#[derive(Debug)]
pub enum Content {
    Bits(u64),
    Child(NodeVec),    // EYE - should be 8 bytes
    Ones
}

// NodeVec methods
impl Content {
    pub fn add_node(&mut self, node: Node) {
        // EYE = need to resize
        match self {
            Content::Child(vec) => { 
                vec.add(node);
            }
            _ => {}
        }
    }
}

impl Clone for Content {
    fn clone(&self) -> Content {
        match self {
            Content::Bits(bits) => Content::Bits(bits.clone()),
            Content::Child(vec) => Content::Child(vec.clone()),
            Content::Ones => Content::Ones,
        }
    }
}
