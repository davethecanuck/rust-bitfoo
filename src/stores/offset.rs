use std::collections::BTreeSet;

#[derive(Debug)]
pub struct OffsetStore {
    offsets: BTreeSet<u16>,
}

// Public interface
impl OffsetStore {
    // Create with short u64 vector
    pub fn new() -> Self {
        OffsetStore{ 
            offsets: BTreeSet::<u16>::new()
        }
    }

    // Create a boxed instance
    pub fn box_new() -> Box<Self> {
        Box::new(Self::new())
    }

    // Set a bit
    pub fn set(&mut self, bitno: u16) {
        self.offsets.insert(bitno);
    }

    // Clear a bit
    pub fn clear(&mut self, bitno: u16) {
        self.offsets.remove(&bitno);
    }

    // Return the boolean value of a bit
    pub fn get(&self, bitno: u16) -> bool {
        self.offsets.contains(&bitno)
    }
}
