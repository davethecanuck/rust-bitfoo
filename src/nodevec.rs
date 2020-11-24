use crate::Node;

// Using a Vec wrapper so we can enforce max size
// of 2^8 and implement custom resizing algo
// Also using Box to make size 8 bytes on the stack

#[derive(Debug)]
pub struct NodeVec {
    vec: Box<Vec<Node>>,   
    // NOTE: Box Reduces size in Content enum
    // but is an extra level of indirection.
}

impl NodeVec {
    pub fn new() -> Self {
        NodeVec {
            vec: Box::new(Vec::with_capacity(1))
        }
    }

    pub fn capacity(&self) -> usize {
        self.vec.capacity()
    }

    pub fn push(&mut self, node: Node) {
        // EYE - should manually expand capacity
        self.vec.push(node);
    }

    pub fn insert(&mut self, node: Node, offset: u8) {
        // EYE - check capacity
        self.vec.insert(offset as usize, node);
    }

    // Return the node which matches this key.
    // If not found, return error with the offset
    // where it would need to be located.
    pub fn search(&self, key: u8) -> Result<(u8,&Node),u8> {
        match self.vec.binary_search_by_key(&key, |node| node.key()) {
            Ok(offset) => Ok((offset as u8, &self.vec[offset])),
            Err(offset) => Err(offset as u8)
        }
    }

    pub fn search_mut(&mut self, key: u8) -> Result<(u8,&mut Node),u8> {
        match self.vec.binary_search_by_key(&key, |node| node.key()) {
            Ok(offset) => Ok((offset as u8, &mut self.vec[offset])),
            Err(offset) => Err(offset as u8)
        }
    }

    pub fn get(&self, offset: u8) -> Result<&Node,u8> {
        if (offset as usize) < self.vec.len() {
            Ok(&self.vec[offset as usize])
        }
        else {
            Err(offset as u8)
        }
    }

    pub fn get_mut(&mut self, offset: u8) -> Result<&Node,u8> {
        if (offset as usize) < self.vec.len() {
            Ok(&mut self.vec[offset as usize])
        }
        else {
            Err(offset as u8)
        }
    }
}

impl Clone for NodeVec {
    fn clone(&self) -> NodeVec {
        NodeVec { 
            vec: Box::new(self.vec.to_vec())
        }
    }
}
