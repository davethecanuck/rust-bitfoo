use std::ops::{Index,BitAnd,BitOr};
use std::iter::Iterator;

// Static 256 bit vector
#[derive(Debug)]
pub struct BitVec256 {
    data: [u64;4],
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

    pub fn is_empty(&self) -> bool {
        self.data[0] | self.data[1] | self.data[2] | self.data[3] == 0
    }

    pub fn is_full(&self) -> bool {
        self.data[0] & self.data[1] & self.data[2] & self.data[3] == u64::MAX
    }

    // Return an iterator
    pub fn iter(&self) -> BitVec256Iterator {
        BitVec256Iterator {
            vec: self,
            bitno: 0,
        }
    }

    pub fn raw_data(&self, offset: u8) -> u64 {
        self.data[offset as usize]
    }
}

// Private interface
impl BitVec256 {
    fn location(&self, bitno: u8) -> (u8,u8) {
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

// Static references for [] return values
static TRUE: bool = true;
static FALSE: bool = false;

// Override [] operator
impl Index<u8> for BitVec256 {
    type Output = bool;

    fn index(&self, bitno: u8) -> &Self::Output {
        match self.get(bitno) {
            true => &TRUE,
            false => &FALSE
        }
    }
}

// Override & operator
impl BitAnd for BitVec256 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = BitVec256::new();
        for i in 0..rhs.data.len() {
            result.data[i] = self.data[i] & rhs.data[i];
        }
        result
    }
}

// Override & operator
impl BitOr for BitVec256 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = BitVec256::new();
        for i in 0..rhs.data.len() {
            result.data[i] = self.data[i] | rhs.data[i];
        }
        result
    }
}

// Iterator over set bits
pub struct BitVec256Iterator<'a> {
    vec: &'a BitVec256,
    bitno: u8,
}

impl<'a> Iterator for BitVec256Iterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.bitno == u8::MAX {
            return None;
        }

        while self.bitno <= u8::MAX {
            let currbit = self.bitno;
            if self.bitno < u8::MAX {
                self.bitno += 1;
            }

            let (word, offset) = self.vec.location(currbit);
            if self.vec.raw_data(word) >> offset & 1 > 0 {
                return Some(currbit);
            }
        }

        // No more found if we got here
        None
    }
}

#[cfg(test)]
#[path = "./bitvec256_test.rs"]
mod tests;
