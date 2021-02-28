use std::fmt;
use crate::bitvec256::iter::BitVec256Iterator;

// Static 256 bit vector
//#[derive(Debug)]
pub struct BitVec256 {
    pub (super) data: [u64;4],
}

// Public interface
impl BitVec256 {
    // Constructor initializes to 0/false
    pub fn new() -> Self {
        BitVec256 {
            data: [0;4]
        }
    }

    // Use u8 as offset so we don't need to do
    // any bounds checking
    pub fn set(&mut self, bitno: u8) {
        let (word, offset) = self.location(bitno);
        self.data[word as usize] |= 1 << offset;
    }

    pub fn clear(&mut self, bitno: u8) {
        let (word, offset) = self.location(bitno);
        self.data[word as usize] &= !(1 << offset);
    }

    pub fn get(&self, bitno: u8) -> bool {
        let (word, offset) = self.location(bitno);
        (self.data[word as usize] & (1 << offset)) > 0
    }

    pub fn set_all(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = u64::MAX;
        }
    }

    pub fn clear_all(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = 0;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data[0] == 0 && self.data[1] == 0 &&
            self.data[2] == 0 && self.data[3] == 0
    }

    pub fn is_full(&self) -> bool {
        // EYE - should do lazy check
        self.data[0] & self.data[1] & self.data[2] & self.data[3] == u64::MAX
    }

    // Return the offset for the given key. This is the
    // number of bits set before our key. If key is not
    // set, return Err with the offset
    pub fn offset(&self, key: u8) -> Result<u8,u8> {
        let mut offset:u8 = 0;
        for currkey in self.iter() {
            if currkey == key {
                // Found our key
                return Ok(offset);
            }
            else if currkey > key {
                // We stepped past our key
                break;
            }

            offset += 1;
        }

        // Our key is not set
        Err(offset)
    }

    // Return an iterator
    pub fn iter(&self) -> BitVec256Iterator {
        BitVec256Iterator {
            vec: self,
            bitno: 0,
            wordno: 0,
        }
    }

    // EYE - revisit and perhaps make pub (super) only
    pub fn raw_data(&self, offset: u8) -> u64 {
        self.data[offset as usize]
    }
}

// Private interface
impl BitVec256 {
    pub (super) fn location(&self, bitno: u8) -> (u8,u8) {
        (bitno / 64, bitno % 64)
    }
}

// Clone interface
impl Clone for BitVec256 {
    fn clone(&self) -> BitVec256 {
        BitVec256 {
            data: self.data.clone(),
        }
    }
}

// Debug interface
impl fmt::Debug for BitVec256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ 3:[{:#x}], 2:[{:#x}], 1:[{:#x}], 0:[{:#x}] ]", 
               self.data[3], self.data[2], self.data[1], self.data[0])
    }
}