use bitvec::prelude as bv;

#[derive(Debug)]
pub struct BitVecStore {
    vec: bv::BitVec
}

// Public interface
impl BitVecStore {
    // Create with short u64 vector
    pub fn new() -> Self {
        BitVecStore{ 
            vec: bv::BitVec::<bv::LocalBits, usize>::with_capacity(1),
        }
    }

    // Create a boxed instance
    pub fn box_new() -> Box<Self> {
        Box::new(Self::new())
    }

    // EYE - This has resize method to shrink capacity to length
    
    // Set a bit
    pub fn set(&mut self, bitno: u16) {
        self.add_capacity(bitno);
        self.vec.set(bitno as usize, true);
    }

    // Clear a bit
    pub fn clear(&mut self, bitno: u16) {
        self.add_capacity(bitno);
        self.vec.set(bitno as usize, false);
    }

    // Return the boolean value of a bit
    pub fn get(&self, bitno: u16) -> bool {
        // If missing, it's false
        match self.vec.get(bitno as usize) {
            Some(b) => *b,
            _ => false
        }
    }
}

// Private interface
impl BitVecStore {
    fn add_capacity(&mut self, bitno: u16) {
        for _i in self.vec.len()..=bitno as usize {
            self.vec.push(false);
        }
    }
}

// Implement the clone trait
impl Clone for BitVecStore {
    fn clone(&self) -> Self {
        let bitvec = self.vec.clone();
        BitVecStore{ vec: bitvec }
    }
}
