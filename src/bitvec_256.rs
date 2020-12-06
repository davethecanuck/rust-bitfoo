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

//==============================================
// Unit tests
//==============================================

#[cfg(test)]
mod tests {
    use crate::BitVec256;

    #[test]
    fn location() {
        let v = BitVec256::new();
        assert_eq!(v.location(63), (0,63));
        assert_eq!(v.location(249), (3,57));
        assert_eq!(v.location(255), (3,63));
    }

    #[test]
    fn clone() {
        let mut v = BitVec256::new();
        v.set(0);
        v.set(1);
        let mut v2 = v.clone();
        v.clear(0);
        v2.set(2); 

        // Only the first byte of word 0 
        // has bits set
        assert_eq!(v.raw_data(0),  0b_0000_0010);
        assert_eq!(v2.raw_data(0), 0b_0000_0111);
    }

    #[test]
    fn raw_data() {
        let mut v = BitVec256::new();

        // Set bits amongst each of the words
        for bitno in [0,65,130,254,255].iter() {
            v.set(*bitno);
        }
        assert_eq!(v.raw_data(0), 0b_0000_0001);
        assert_eq!(v.raw_data(1), 0b_0000_0010);
        assert_eq!(v.raw_data(2), 0b_0000_0100);
        assert_eq!(v.raw_data(3), 0b_1100_0000 << 7*8);

        // Clear some bits
        v.clear(0);
        v.clear(255);
        assert_eq!(v.raw_data(0), 0b_0000_0000);
        assert_eq!(v.raw_data(3), 0b_0100_0000 << 7*8);
    }

    #[test]
    fn index() {
        let mut v = BitVec256::new();
        assert_eq!(false, v[0]);
        v.set(0);
        assert_eq!(true, v[0]);
        v.set(1);
        assert_eq!(true, v[1]);
        v.set(2);
        assert_eq!(true, v[2]);
        v.clear(2);
        assert_eq!(false, v[2]);
        v.set(255);
        assert_eq!(true, v[255]);
    }

    #[test]
    fn iterator() {
        // Populate a vector
        let mut v = BitVec256::new();
        let input_bits = vec![0_u8, 28, 65, 129, 255];
        for i in 0..input_bits.len() {
            v.set(input_bits[i]);
        }

        // Now iterate throuh the vector and 
        // check that it matches the input
        let mut output_bits = Vec::new();
        for b in v.iter() {
            output_bits.push(b);
        }
        assert_eq!(&input_bits, &output_bits);
    }

    #[test]
    fn get_set_and_clear() {
        let mut v = BitVec256::new();
        assert_eq!(true, v.is_empty());

        let max_bit = 13;
        let set_bit = 5;
        let clear_bit = 10;

        // Set a bunch of bits and then test 
        // after all are set
        for bitno in 0..=max_bit {
            // Should start off all 0/false
            assert_eq!(v.get(bitno), false);
            
            if bitno % set_bit == 0 {
                v.set(bitno);
                if bitno % clear_bit == 0 {
                    v.clear(bitno);
                }
            }
        }
        assert_eq!(false, v.is_empty());

        for bitno in 0..=max_bit {
            if bitno % set_bit == 0 {
                if bitno % clear_bit == 0 {
                    assert_eq!(false, v.get(bitno));
                }
                else {
                    assert_eq!(true, v.get(bitno));
                }
            }
            else {
                // All others were not set
                assert_eq!(false, v.get(bitno));
            }
        }
    }

    #[test]
    fn empty_and_full() {
        let mut v = BitVec256::new();
        assert_eq!(true, v.is_empty());
        assert_eq!(false, v.is_full());

        // Now fill the vector 
        for bitno in 0..=u8::MAX {
            assert_eq!(false, v.is_full());
            v.set(bitno);
            assert_eq!(false, v.is_empty());
        }
        assert_eq!(false, v.is_empty());
        assert_eq!(true, v.is_full());
    }

    #[test]
    fn bitand() {
        let mut a = BitVec256::new();
        let mut b = BitVec256::new();
        a.set(0);
        a.set(1);
        b.set(1);
        let c = a & b;
        assert_eq!(false, c[0]); 
        assert_eq!(true, c[1]); 
        assert_eq!(0b_0010, c.data[0]); 
    }

    #[test]
    fn bitor() {
        let mut a = BitVec256::new();
        let mut b = BitVec256::new();
        a.set(0);
        b.set(0);
        b.set(1);
        let c = a | b;
        assert_eq!(true, c[0]); 
        assert_eq!(true, c[1]); 
        assert_eq!(0b_0011, c.data[0]); 
    }
}
