use std::vec::Vec;

mod foobox;
use foobox::FooBox;

struct BitMapBox {
    words: Vec<u64>,
}

impl BitMapBox {
    // Create with short u64 vector
    pub fn new() -> Self {
        BitMapBox{ 
            words: Vec::<u64>::with_capacity(1),
        }
    }
}

impl FooBox for BitMapBox {
    // Shrink allocated space 
    fn shrink(&mut self) {
        self.offsets.shrink_to_fit();
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

fn main() {
    let mut bm = BitMapBox::new();

    for i in 0..=65 {
        if i % 3 == 0 {
            // Set every 3rd bit
            bm.set(i);
        }
        if i % 2 == 0 {
            // But clear every 2nd bit
            bm.clear(i);
        }
        println!("SETTING: Is {} set ? => {} (len={}, capacity={})", 
                 i, bm.get(i), bm.words.len(), bm.words.capacity());
    }

    for i in 0..=128 {
        println!("CHECKING: {} set ? => {} (len={}, capacity={})", 
                 i, bm.get(i), bm.words.len(), bm.words.capacity());
    }

    bm.shrink();
    println!("AFTER SHRINK: len={}, capacity={}", bm.words.len(), bm.words.capacity());
}
