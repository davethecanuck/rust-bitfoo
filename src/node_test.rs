#[cfg(test)]
use crate::{Node,Addr};

#[test]
fn index() {
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

