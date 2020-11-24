use crate::{Node,NodeVec};

#[derive(Debug)]
pub enum Content {
    // EYE - Even with boxed NodeVec this is 16 bytes.
    // Is there another way?
    Bits(u64),
    Child(NodeVec),
    Ones
}

// NodeVec methods
impl Content {
    pub fn add_node(&mut self, node: Node) {
        // EYE = need to resize vector more intelligently
        match self {
            Content::Child(vec) => { 
                vec.push(node);
            }
            _ => {}
        }
    }
}

impl Clone for Content {
    fn clone(&self) -> Content {
        match self {
            Content::Bits(bits) => Content::Bits(bits.clone()),
            Content::Child(nv) => Content::Child(nv.clone()),
            Content::Ones => Content::Ones,
        }
    }
}
