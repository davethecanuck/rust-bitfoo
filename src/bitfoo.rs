use crate::{MapStore,OffsetStore,BitVecStore};

/*
 * Define the types of stores that can hold a bit vector.
 * Wrap all objects in Box so size is usize for all enum
 * instances (biggest instance sets the enum size).
 */
#[derive(Debug)]
pub enum BitFoo {
    Map(Box<MapStore>),
    Offset(Box<OffsetStore>),
    Vec(Box<BitVecStore>),
    Zero,
    One,
}

impl BitFoo {
    // Bitwise operations
    pub fn set(&mut self, bitno: u16) {
        match self {
            BitFoo::Map(v) => v.set(bitno),
            BitFoo::Offset(v) => v.set(bitno),
            BitFoo::Vec(v) => v.set(bitno),
            _ => {}
        }
    }

    pub fn clear(&mut self, bitno: u16) {
        match self {
            BitFoo::Map(s) => s.clear(bitno),
            BitFoo::Offset(s) => s.clear(bitno),
            BitFoo::Vec(s) => s.clear(bitno),
            _ => {}
        };
    }

    pub fn get(&self, bitno: u16) -> bool {
        match self {
            BitFoo::Map(s) => s.get(bitno),
            BitFoo::Offset(s) => s.get(bitno),
            BitFoo::Vec(s) => s.get(bitno),
            BitFoo::Zero => false,
            BitFoo::One => true,
        }
    }

    // Implement logical operations with another container
    pub fn and(&self, other: &BitFoo) -> BitFoo {
        /*
        match self {
            BitFoo::Zero => BitFoo::Zero,
            BitFoo::One => other.clone(),
            BitFoo::Vec => {
                match other {
                    BitFoo::Zero => BitFoo::Zero,
                    BitFoo::One => self.clone(),
                    BitFoo::Vec => {}
                },
            }
        }
        */
        BitFoo::Zero
    }
}

