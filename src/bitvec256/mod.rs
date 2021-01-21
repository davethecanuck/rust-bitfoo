mod bitvec256;
mod ops;
mod iter;

pub use self::bitvec256::BitVec256;
pub use self::iter::BitVec256Iterator;

#[cfg(test)]
mod tests;