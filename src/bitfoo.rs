use crate::{MapStore,OffsetStore,BitVecStore};

/*
 * Define the types of stores that can hold a bit vector.
 * Wrap all objects in Box so size is usize for all enum
 * instances (biggest instance sets the enum size).
 */
#[derive(Debug)]
pub enum BitFoo {
    Offset(Box<OffsetStore>),
    Vec(Box<BitVecStore>),
    Zero,
    One,
}

impl BitFoo {
    // Bitwise operations
    pub fn set(&mut self, bitno: u16) {
        match self {
            BitFoo::Offset(v) => v.set(bitno),
            BitFoo::Vec(v) => v.set(bitno),
            _ => {}
        }
    }

    pub fn clear(&mut self, bitno: u16) {
        match self {
            BitFoo::Offset(s) => s.clear(bitno),
            BitFoo::Vec(s) => s.clear(bitno),
            _ => {}
        };
    }

    pub fn get(&self, bitno: u16) -> bool {
        match self {
            BitFoo::Offset(s) => s.get(bitno),
            BitFoo::Vec(s) => s.get(bitno),
            BitFoo::Zero => false,
            BitFoo::One => true,
        }
    }
}

impl Clone for BitFoo {
    fn clone(&self) -> BitFoo {
        match self {
            BitFoo::Offset(offset_store) => BitFoo::Offset(offset_store.clone()),
            BitFoo::Vec(vec_store) => BitFoo::Vec(vec_store.clone()),
            BitFoo::Zero => BitFoo::Zero,
            BitFoo::One => BitFoo::One,
        }
    }
}
