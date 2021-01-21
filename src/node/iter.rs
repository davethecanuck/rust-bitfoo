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

struct RunIterator {
}

impl RunIterator {
    fn new(key: u8) -> RunIterator {
        RunIterator {}
    }
}

struct BitsIterator {
}

impl BitsIterator {
    fn new(bits: u64) -> BitsIterator {
        BitsIterator {}
    }
}

pub struct NodeIterator<'a> {
    addr: Addr,
    node: &'a Node,
    index_iter: KeyIndexIterator<'a>,
    key_state: Option<KeyState>,
    child_iter: ChildIterator<'a>,
}

impl<'a> NodeIterator<'a> {
    pub fn new(node: &'a Node, addr: Addr) -> NodeIterator {
        let mut index_iter = node.index.iter();
        let key_state = index_iter.next();

        let child_iter = match key_state {
            Some(KeyState::Node(key, offset)) => {
                // Iterator for child node
                match &node.content {
                    Content::Bits(vec) => {
                        let child_bits = vec[offset];
                        ChildIterator::Bits(BitsIterator::new(child_bits))
                    },
                    Content::Nodes(vec) => {
                        let child_node = &vec[offset];
                        ChildIterator::Node(
                            Box::new(
                                child_node.iter(addr.clone())
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
    
        NodeIterator {
            addr: addr, 
            node: node,
            index_iter, 
            key_state, 
            child_iter,
        }
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = Addr;
    // EYE - Should pass an Addr around, but return
    // u64 (bitno). This simplifies iterating through
    // runs

    fn next(&mut self) -> Option<Self::Item> {
        // EYE - Need Addr methods to show the range
        // of values for a given Addr prefix and level
        // - do we need different types of iterators?
        let mut result = None;

        match self.key_state {
            Some(KeyState::Node(key, offset)) => {
                match &self.node.content {
                    Content::Nodes(_vec) => {
                        // Get iterator for the child
                        let mut child_addr = self.addr.clone();
                        child_addr.set(self.node.level() - 1, key);
                        // EYE - how do we get back here...
                        // - need to map this out
                        // - Can't have iterator containing iterator
                        // as size is undefined (unless we box)
                        // 
                        // EYE - Use a NodeContentIterator enum which 
                        // has iterators for Run, Bits, or Nodes
                        // - Consider putting modules into src/node/node.rs
                        //   and src/node/node_test.rs, .../node_iter.rs, etc
                        // - This gives us private node::iter and node::test submodules
                    },
                    Content::Bits(_vec) => {
                        // EYE - basically need a Bits iterator...
                    },
                }

                // EYE need to iterate on the child
            },
            Some(KeyState::Run(key)) => {
                // Use Addr method to get start and
                // end bitno. Keep this in the iterator
                // and iterate over the bits.
            },
            _ => ()
        }
        result
    }
}
