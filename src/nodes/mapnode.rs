use std::vec::Vec;
use crate::traits::BitSetLogic;

pub struct MapNode {
    words: Vec<u64>,
}

impl MapNode {
    // Create with short u64 vector
    pub fn new() -> Self {
        MapNode{ 
            words: Vec::<u64>::with_capacity(1),
        }
    }

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

impl BitSetLogic for MapNode {
    // Shrink allocated space 
    fn shrink(&mut self) {
        self.words.shrink_to_fit();
    }
    
    // Set a bit
    fn set(&mut self, bitno: u16) {
        let (wordoff, bitoff) = self.location(bitno);
        self.resize(wordoff);
        let value = self.words[wordoff as usize];
        self.words[wordoff as usize] = value | (1 << bitoff);
    }

    // Clear a bit
    fn clear(&mut self, bitno: u16) {
        // Make sure we have space allocated
        let (wordoff, bitoff) = self.location(bitno);
        self.resize(wordoff);
        let value = self.words[wordoff as usize];
        self.words[wordoff as usize] = value & !(1 << bitoff);
    }

    // Return the boolean value of a bit
    fn get(&self, bitno: u16) -> bool {
        let (wordoff, bitoff) = self.location(bitno);
        if self.words.len() <= wordoff as usize {
            false
        }
        else {
            ((self.words[wordoff as usize] >> bitoff) & 1) == 1
        }
    }
}
