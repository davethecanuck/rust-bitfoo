/*
 * Define logical operations that must be implemented by
 * a BitStore type
 */
pub trait BitStore {
    // Shrinks underlying store to release unused capacity
    fn shrink(&mut self);

    // Bitwise operations
    fn get(&self, bitno: u16) -> bool;
    fn set(&mut self, bitno: u16);
    fn clear(&mut self, bitno: u16);

    // Grab the next u64 chunk of bits
    fn get_u64(&self, offset: u16) -> u64;

    // Implement logical operations with another container
    //fn and(&self, other: &dyn BitFoo) -> Box<dyn BitFoo>;
}

