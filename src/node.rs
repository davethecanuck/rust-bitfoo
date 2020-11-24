use crate::Content;

#[derive(Debug)]
pub struct Node {
    pub key: u8,
    pub content: Content,
}

impl Node {
    pub fn new(key: u8, content: Content) -> Self {
        Node {
            key, 
            content,
        }
    }

    pub fn key(&self) -> u8 {
        self.key
    }
}

impl Clone for Node {
    fn clone(&self) -> Node {
        Node { 
            key: self.key.clone(),
            content: self.content.clone(),
        }
    }
}
