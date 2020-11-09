use std::vec::Vec;

#[derive(Debug)]
pub struct MapStore {
    words: Vec<u64>,
}

// Private methods
impl MapStore {
    fn location(&self, bitno: u16) -> (u16,u16) {
        (bitno/16, bitno%16)
    }

    fn resize(&mut self, wordoff: u16) {
        // EYE - TBD pre-allocate capacity as per
        // the roaring growth algo
        while self.words.len() <= wordoff as usize {
            self.words.push(0);
        }
    }
}

// Public interface
impl MapStore {
    // Create with short u64 vector
    pub fn new() -> Self {
        MapStore{ 
            words: Vec::<u64>::with_capacity(1),
        }
    }

    // Create a boxed instance
    pub fn box_new() -> Box<Self> {
        Box::new(Self::new())
    }

    // Shrink allocated space to minimum
    pub fn shrink(&mut self) {
        self.words.shrink_to_fit();
    }

    // Set a bit
    pub fn set(&mut self, bitno: u16) {
        let (wordoff, bitoff) = self.location(bitno);
        self.resize(wordoff);
        let value = self.words[wordoff as usize];
        self.words[wordoff as usize] = value | (1 << bitoff);
    }

    // Clear a bit
    pub fn clear(&mut self, bitno: u16) {
        // Make sure we have space allocated
        let (wordoff, bitoff) = self.location(bitno);
        self.resize(wordoff);
        let value = self.words[wordoff as usize];
        self.words[wordoff as usize] = value & !(1 << bitoff);
    }

    // Return the boolean value of a bit
    pub fn get(&self, bitno: u16) -> bool {
        let (wordoff, bitoff) = self.location(bitno);
        if self.words.len() <= wordoff as usize {
            false
        }
        else {
            ((self.words[wordoff as usize] >> bitoff) & 1) == 1
        }
    }

    // Return a u64 chunk from the given u16 offset
    // - Used for stepping though array in 64-bit chunks
    pub fn get_u64(&self, offset: u16) -> u64 {
        if offset as usize <= self.words.len() {
            self.words[offset as usize]
        }
        else {
            0
        }
    }
}
