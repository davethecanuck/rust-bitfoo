pub trait FooBox {
    // Need a default constructor
    //fn new() -> Self;

    // Set a bit (to 1)
    fn set(&mut self, bitno: u16);

    // Clear a bit (to 0)
    fn clear(&mut self, bitno: u16);

    // Return true if bit is set
    fn get(&self, bitno: u16) -> bool;

    // Shrinks underlying store to release unused capacity
    fn shrink(&mut self);

    // Implement logical 'and' with a BitMapBox
    /*
    fn and(&self, other: &dyn FooBox) -> Self;
    */

    // EYE TBD use std::ops to do operator overloading
}

