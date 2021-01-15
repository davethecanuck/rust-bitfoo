use std::iter::Iterator;
use crate::BitVec256;

// Iterator over set bits
pub struct BitVec256Iterator<'a> {
    pub (super) vec: &'a BitVec256,
    pub (super) bitno: u8,
    pub (super) wordno: u8,
}

impl<'a> Iterator for BitVec256Iterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // Count leading zeros and shift until
        // no more found
        while self.wordno < 4 {
            let word = self.vec.data[self.wordno as usize] >> self.bitno;
            let offset = word.trailing_zeros() as u8;

            if offset >= 64 {
                // Done this word - on to the next
                self.wordno += 1;
                self.bitno = 0;
            }
            else {
                // Calculate the return value
                let currbit = self.bitno + offset;
                let retval = currbit + (self.wordno * 64);

                // Increment our iterator bitno/wordno
                self.bitno = currbit + 1;
                if self.bitno >= 64 {
                    self.wordno += 1;
                    self.bitno = 0;
                }
                return Some(retval);
            }
        }

        // No more found if we got here
        None
    }
}
