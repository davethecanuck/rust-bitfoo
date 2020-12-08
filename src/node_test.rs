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
fn index() {
    // Test the [] operator
    let mut node = Node::new(1);
    let input_bits = [0_64, 10];
    for bitno in &input_bits {
        assert_eq!(false, node[*bitno]);
        node.set(&Addr::new(*bitno));
        assert_eq!(true, node[*bitno]);
    }
}
