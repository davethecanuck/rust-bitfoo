use std::{iter::Iterator};
use crate::bitvec64;

// Iterator for BitVec64 bits
pub struct BitVec64Iterator {
    bitvec: u64,
    curr_bitno: u8,
}

impl BitVec64Iterator {
    pub fn new(bitvec: u64) -> BitVec64Iterator {
        BitVec64Iterator {
            bitvec,
            curr_bitno: 0,
        }
    }
}

impl Iterator for BitVec64Iterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_bitno > bitvec64::MAX_BIT {
            // The previous set bit was the last one
            return None;
        }
        else {
            let bits:u64 = self.bitvec >> self.curr_bitno;
            let trailing_zeros = bits.trailing_zeros();

            if trailing_zeros > bitvec64::MAX_BIT as u32 {
                // All 0's
                None
            }
            else {
                let retval = trailing_zeros as u8 + self.curr_bitno;
                self.curr_bitno = retval + 1;
                Some(retval)
            }
        }
    }
}