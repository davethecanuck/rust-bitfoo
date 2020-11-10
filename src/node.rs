use crate::{BitFoo,OffsetStore};

// Node in our store contains the key (first 2 bytes of bitno) 
// and cardinality (number of bits)
// - EYE - possibly make generic with option of u8, u16, or u32 key
pub struct Node {
    pub key: u16,
    pub cardinality: u16,
    pub store: BitFoo,
}

impl Node {
    // Default constructor uses and offset store
    pub fn new(key: u16) -> Self {
        Node {
            key, 
            cardinality: 0,
            //store: BitFoo::Offset(OffsetStore::box_new()),
            store: BitFoo::Offset(Box::new(OffsetStore::new())),
        }
    }

    // Implement logical operations with another Node
    pub fn and(&self, other: &Node) -> Node {
        let result_store = match &self.store {
            // EYE stubbing in
            BitFoo::Zero => BitFoo::Zero,
            BitFoo::One => other.store.clone(),
            BitFoo::Vec(vec_store) => {
                match other.store {
                    // Zero & Zero
                    BitFoo::Zero => BitFoo::Zero,

                    // Vec & One
                    BitFoo::One => other.store.clone(),

                    // Default
                    // EYE - Need to actually implement the 'and' function
                    _ => BitFoo::Zero,
                }
            },

            // Default
            _ => BitFoo::Zero,
        };

        Node {
            key: self.key,
            cardinality: self.cardinality,
            store: result_store,
        }
    }

}
