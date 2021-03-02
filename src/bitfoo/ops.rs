use std::ops::Index;
use crate::BitFoo;

// Static references for [] return values
static TRUE: bool = true;
static FALSE: bool = false;

// Implement [u64] operator
impl Index<u64> for BitFoo {
    type Output = bool;

    fn index(&self, bitno: u64) -> &Self::Output {
        // Can't easily return self.get() as
        // it is a reference to a local var.
        match self.get(bitno) {
            true => &TRUE,
            false => &FALSE
        }
    }
}
