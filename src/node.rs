use crate::Content;

#[derive(Debug)]
pub struct Node {
    pub offset: u8,
    pub content: Content,
}

impl Node {
    pub fn new(offset: u8, content: Content) -> Self {
        Node {
            offset, 
            content
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Node {
        Node { 
            offset: self.offset.clone(),
            content: self.content.clone()
        }
    }
}
