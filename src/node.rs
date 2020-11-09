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
            store: BitFoo::Offset(OffsetStore::box_new()),
        }
    }
}
