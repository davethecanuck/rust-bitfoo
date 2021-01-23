use crate::{Addr,KeyState,KeyIndexIterator};
use std::iter::Iterator;
use super::Node;
use super::Content;

// Define iterators for the 3 types of content
// we may run into. Bits/Node are Node Contents, 
// whereas Run is from a KeyIndex
enum ChildIterator<'a> {
    End,
    Run(RunIterator),
    Bits(BitsIterator),
    Node(Box<NodeIterator<'a>>),
}

// Iterator for run bits
struct RunIterator {
}

impl RunIterator {
    fn new(key: u8) -> RunIterator {
        RunIterator {}
    }
}

impl Iterator for RunIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // EYE TBD
        None
    }
}

// Iterator for raw level 0 bits
struct BitsIterator {
}

impl BitsIterator {
    fn new(bits: u64) -> BitsIterator {
        BitsIterator {}
    }
}

impl Iterator for BitsIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // EYE TBD
        None
    }
}

// Main iterator for a node
pub struct NodeIterator<'a> {
    addr: Addr,
    node: &'a Node,
    index_iter: KeyIndexIterator<'a>,
    child_iter: ChildIterator<'a>,
}

impl<'a> NodeIterator<'a> {
    pub fn new(node: &'a Node, addr: Addr) -> NodeIterator {
        NodeIterator {
            addr, 
            node,
            index_iter: node.index.iter(), 
            child_iter: ChildIterator::End,
        }
    }

    // Return next item from child iterator
    fn next_from_child(&mut self) -> Option<u64> {
        match &mut self.child_iter {
            ChildIterator::Bits(iter) => iter.next(),
            ChildIterator::Run(iter) => iter.next(),
            ChildIterator::Node(iter) => iter.next(),
            ChildIterator::End => None,
        }
    }

    // Return the next child iterator
    fn update_child_iterator(&mut self) {
        self.child_iter = match self.index_iter.next() {
            Some(KeyState::Node(key, offset)) => {
                // Iterator for child node
                match &self.node.content {
                    Content::Bits(vec) => {
                        let child_bits = vec[offset];
                        ChildIterator::Bits(BitsIterator::new(child_bits))
                    },
                    Content::Nodes(vec) => {
                        let child_node = &vec[offset];
                        ChildIterator::Node(
                            Box::new(
                                child_node.iter(self.addr.clone())
                            )
                        )
                    },
                }
            },
            Some(KeyState::Run(key)) => {
                ChildIterator::Run(RunIterator::new(key))
            },
            _ => ChildIterator::End
        };
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    // Returns u64, but internally use Addr
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = self.next_from_child();
        if result == None {
            // We've exhausted the current child iterator
            self.update_child_iterator();
            result = self.next_from_child();
        }
        result
    }
}