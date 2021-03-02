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
    curr_bit: u64,
    end_bit: u64,
}

impl RunIterator {
    fn new(parent_addr: &Addr, key: u8, level: u8) -> RunIterator {
        let mut addr = parent_addr.clone();
        addr.set(level, key);

        RunIterator {
            curr_bit: addr.min_bitno(level), 
            end_bit: addr.max_bitno(level),
        }
    }
}

impl Iterator for RunIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_bit <= self.end_bit {
            let retval = self.curr_bit;
            self.curr_bit += 1;
            Some(retval)
        }
        else {
            None
        }
    }
}

// Iterator for raw level 0 bits
struct BitsIterator {
    start_bit: u64, 
    raw_bits: u64,
    curr_bit: u64,
}

impl BitsIterator {
    fn new(parent_addr: &Addr, key: u8, raw_bits: u64) -> BitsIterator {
        // Level 1 is set to the current key
        let mut addr = parent_addr.clone();
        addr.set(1, key);
        let start_bit = addr.min_bitno(1);

        BitsIterator {
            start_bit, 
            raw_bits,
            curr_bit: 0, // This is relative to the start_bit
        }
    }
}

impl Iterator for BitsIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_bit > 63 {
            // Last iteration incremented us to the end of the word
            None
        }
        else {
            let word = self.raw_bits >> self.curr_bit;
            let trailing_zeros = word.trailing_zeros() as u64;
            let word_bit = self.curr_bit + trailing_zeros;

            if word_bit > 63 {
                // 0's all the way to the most-significant-bit
                return None
            }
            else {
                // Increment our current bit before returning
                let retval = self.start_bit + word_bit;
                self.curr_bit = word_bit + 1;
                Some(retval)
            }
        }
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
                        ChildIterator::Bits(
                            BitsIterator::new(&self.addr, key, child_bits)
                        )
                    },
                    Content::Nodes(vec) => {
                        let child_node = &vec[offset];
                        let mut addr = self.addr.clone();
                        addr.set(self.node.level(), key);

                        ChildIterator::Node(
                            Box::new(
                                NodeIterator::new(child_node, addr)
                            )
                        )
                    },
                }
            },
            Some(KeyState::Run(key)) => {
                ChildIterator::Run(
                    RunIterator::new(&self.addr, key, self.node.level())
                )
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