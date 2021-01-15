use crate::Addr;
use std::ops::Index;
use crate::Node;

// Static references for [] return values
static TRUE: bool = true;
static FALSE: bool = false;

// Implement [u64] operator
impl Index<u64> for Node {
    type Output = bool;

    fn index(&self, bitno: u64) -> &Self::Output {
        // Can't easily return self.get() as
        // it is a reference to a local var.
        let addr = Addr::new(bitno);
        match self.get(&addr) {
            true => &TRUE,
            false => &FALSE
        }
    }
}

// Implement [&Addr] operator
impl Index<&Addr> for Node {
    type Output = bool;

    fn index(&self, addr: &Addr) -> &Self::Output {
        // Can't easily return self.get() as
        // it is a reference to a local var.
        match self.get(addr) {
            true => &TRUE,
            false => &FALSE
        }
    }
}
