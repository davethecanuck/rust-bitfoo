use std::ops::{Index,BitAnd,BitOr};
use crate::BitVec64;

// Static references for [] return values
static TRUE: bool = true;
static FALSE: bool = false;

// Override [] operator
impl Index<u8> for BitVec64 {
    type Output = bool;

    fn index(&self, bitno: u8) -> &Self::Output {
        match self.get(bitno) {
            true => &TRUE,
            false => &FALSE
        }
    }
}

// Override & operator
impl BitAnd for BitVec64 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitVec64(self.raw_data() & rhs.raw_data())
    }
}

// Override & operator for references
impl BitAnd for &BitVec64 {
    type Output = BitVec64;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitVec64(self.raw_data() & rhs.raw_data())
    }
}

// Override | operator
impl BitOr for BitVec64 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitVec64(self.raw_data() | rhs.raw_data())
    }
}

// Override | operator for references
impl BitOr for &BitVec64 {
    type Output = BitVec64;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitVec64(self.raw_data() | rhs.raw_data())
    }
}