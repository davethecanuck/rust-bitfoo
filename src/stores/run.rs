use std::collections::BTreeMap;

#[derive(Debug)]
pub struct RunStore {
    runs: BTreeMap<u16,u16>,
}

// Public interface
impl RunStore {
    // Create with short u64 vector
    pub fn new() -> Self {
        RunStore{ 
            runs: BTreeMap::<u16,u16>::new()
        }
    }

    // Create a boxed instance
    pub fn box_new() -> Box<Self> {
        Box::new(Self::new())
    }

    // Set a bit
    pub fn set(&mut self, bitno: u16) {
        self.runs.insert(bitno);
    }

    // Clear a bit
    pub fn clear(&mut self, bitno: u16) {
        self.runs.remove(&bitno);
    }

    // Return the boolean value of a bit
    pub fn get(&self, bitno: u16) -> bool {
        self.runs.contains(&bitno)
    }
}
