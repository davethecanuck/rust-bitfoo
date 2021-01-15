use std::ops::{Index,BitAnd,BitOr};
use crate::BitVec256;

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

// Override & operator for references
impl BitAnd for &BitVec256 {
    type Output = BitVec256;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = BitVec256::new();
        for i in 0..rhs.data.len() {
            result.data[i] = self.data[i] & rhs.data[i];
        }
        result
    }
}

// Override | operator
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

// Override | operator for references
impl BitOr for &BitVec256 {
    type Output = BitVec256;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = BitVec256::new();
        for i in 0..rhs.data.len() {
            result.data[i] = self.data[i] | rhs.data[i];
        }
        result
    }
}