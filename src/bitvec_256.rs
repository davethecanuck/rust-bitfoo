use std::ops::Index;

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

    // Fixed size
    pub fn max_offset() -> u8 {
        255
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

    // Return direct access to the raw data
    pub fn data(&self) -> &[u64;4] {
        &self.data
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

// Override [] operator
impl Index<u8> for BitVec256 {
    type Output = bool;

    fn index(&self, bitno: u8) -> &Self::Output {
        // Can't easily return self.get() as
        // it is a reference to a local var.
        match self.get(bitno) {
            true => &true,
            false => &false
        }
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
        assert_eq!(v.data()[0],  0b_0000_0010);
        assert_eq!(v2.data()[0], 0b_0000_0111);
    }

    #[test]
    fn raw_data() {
        let mut v = BitVec256::new();

        // Set bits amongst each of the words
        for bitno in [0,65,130,254,255].iter() {
            v.set(*bitno);
        }
        assert_eq!(v.data()[0], 0b_0000_0001);
        assert_eq!(v.data()[1], 0b_0000_0010);
        assert_eq!(v.data()[2], 0b_0000_0100);
        assert_eq!(v.data()[3], 0b_1100_0000 << 7*8);

        // Clear some bits
        v.clear(0);
        v.clear(255);
        assert_eq!(v.data()[0], 0b_0000_0000);
        assert_eq!(v.data()[3], 0b_0100_0000 << 7*8);
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
                    assert_eq!(v.get(bitno), false);
                }
                else {
                    assert_eq!(v.get(bitno), true);
                }
            }
            else {
                // All others were not set
                assert_eq!(v.get(bitno), false);
            }
        }
    }
}

