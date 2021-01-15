use crate::{Addr,KeyState,KeyIndexIterator};
use std::iter::Iterator;
use crate::Node;
use super::Content;

pub struct NodeIterator<'a> {
    pub (super) node: &'a Node,
    pub (super) index_iter: KeyIndexIterator<'a>,
    pub (super) key_state: Option<KeyState>,
    pub (super) addr: Addr,
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
