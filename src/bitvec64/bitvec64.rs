use std::fmt;
use crate::bitvec64::iter::BitVec64Iterator;

// Max size of BitVec64
pub const MAX_BIT:u8 = 63 as u8;

// Simple tuple struct to give bit vector semantics to u64
#[derive(Clone,Copy)]
pub struct BitVec64(pub u64);

// Public interface
// EYE - panic on bitno > MAX_BIT?
impl BitVec64 {
    pub fn set(&mut self, bitno: u8) {
        self.0 |= 1 << bitno;
    }

    pub fn clear(&mut self, bitno: u8) {
        self.0 &= !(1 << bitno);
    }

    pub fn get(&self, bitno: u8) -> bool {
        (self.0 & (1 << bitno)) > 0
    }

    pub fn set_all(&mut self) {
        self.0 = u64::MAX;
    }

    pub fn clear_all(&mut self) {
        self.0 = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0 
    }

    pub fn is_full(&self) -> bool {
        self.0 == u64::MAX
    }

    pub fn iter(&self) -> BitVec64Iterator {
        BitVec64Iterator::new(self.0)
    }

    pub fn raw_data(&self) -> u64 {
        self.0
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
}

// Clone interface
/* EYE - might use derived clone 
impl Clone for BitVec64 {
    fn clone(&self) -> BitVec64 {
        BitVec64(self.0)
    }
}
*/

// Debug interface
impl fmt::Debug for BitVec64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:#b}]", self.0)
    }
}