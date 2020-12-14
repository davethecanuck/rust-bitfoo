#[cfg(test)]
use crate::{Node,Addr};

#[test]
fn bit_index() {
    // Set a few bits, check index, then clear and check
    let mut node = Node::new(1);
    assert_eq!(node.index.raw_data(0), 0b_0000);
    node.set(&Addr::new(0));
    assert_eq!(node.index.raw_data(0), 0b_0001);
    node.set(&Addr::new(5));
    assert_eq!(node.index.raw_data(0), 0b_0001);
    node.clear(&Addr::new(0));
    assert_eq!(node.index.raw_data(0), 0b_0001);
    node.clear(&Addr::new(5));
    assert_eq!(node.index.raw_data(0), 0b_0000);
}

#[test]
fn set_clear_and_index() {
    for level in 1..=8 {
        // Test some boundary bits in each node level
        let mut node = Node::new(level);
        let bit_mult = 256 * (level as u64 - 1);  // 256 u64's at each level
        let input_bits = [ 0_64, 0 * bit_mult, (64*256-1) * bit_mult ];

        for bitno in &input_bits {
            let curr_bit = *bitno as u64;
            println!("Testing bit {:#b}", curr_bit);
            assert_eq!(false, node[curr_bit]);
            node.set(&Addr::new(curr_bit));
            assert_eq!(true, node[curr_bit]);
            node.clear(&Addr::new(curr_bit));
            assert_eq!(false, node[curr_bit]);
        }
    }
}
