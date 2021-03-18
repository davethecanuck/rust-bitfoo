mod bitvec64;
mod ops;
mod iter;

pub use self::bitvec64::BitVec64;
pub use self::bitvec64::MAX_BIT;
pub use self::iter::BitVec64Iterator;

#[cfg(test)]
mod tests;