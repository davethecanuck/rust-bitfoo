pub trait BitSetLogic {
    // Set a bit (to 1)
    fn set(&mut self, bitno: u16);

    // Clear a bit (to 0)
    fn clear(&mut self, bitno: u16);

    // Return true if bit is set
    fn get(&self, bitno: u16) -> bool;

    // Shrinks underlying store to release unused capacity
    fn shrink(&mut self);

    // Implement logical 'and' with another node
    /*
    fn and(&self, other: &dyn Node) -> Box<dyn Node>;
    fn or(&self, other: &dyn Node) -> Box<dyn Node>;
    fn xor(&self, other: &dyn Node) -> Box<dyn Node>;
    fn not(&self) -> Box<dyn Node>;
    */

    // EYE TBD use std::ops to do operator overloading
    // - Create default implementation based on the 
    // logical operators defined above
}

