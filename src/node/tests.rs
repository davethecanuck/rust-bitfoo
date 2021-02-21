use std::vec::Vec;

#[cfg(test)]
use crate::{Node,Addr};

#[test]
fn node_index_l1() {
    // Verify index is empty on node creation
    let mut node = Node::new(1);
    assert_eq!(node.index.is_nodes_empty(), true);
    assert_eq!(node.index.is_runs_empty(), true);

    // Set a few bits, including the ends of the level 1 node
    // (256 * 64 bits)
    let bits = vec![0_u64, 2, 128, 64*256-1];
    let keys:Vec<u8> = bits.iter().map(|b| Addr::new(*b).key(1)).collect();

    for b in &bits {
        let addr = Addr::new(*b);
        node.set(&addr);
    }
    
    // Nodes should be non-empty, but runs is empty
    assert_eq!(node.index.is_nodes_empty(), false);
    assert_eq!(node.index.is_nodes_full(), false);
    assert_eq!(node.index.is_runs_empty(), true);
    assert_eq!(node.index.is_runs_full(), false);
    
    // Run through all of the level 1 node bits and check state
    for k in 0_u8..=255 {    // child keys
        for b in 0..=63 { // words at each key
            let bitno:u64 = (k as u64)*64 + b;
            let addr = Addr::new(bitno);
            assert_eq!(node.index.is_node(&addr), keys.contains(&k));
        }
    }
}
        
#[test]
fn node_clear_all_l1() {
    // Clear all
    let mut node = Node::new(1);
    for k in 0_u8..=255 {
        for b in 0..=63 { 
            let bitno:u64 = (k as u64)*64 + b;
            let addr = Addr::new(bitno);
            node.clear(&addr);
            
            // After last bit is cleared in the word, 
            // the node index should be cleared
            if b == 63 {
                assert_eq!(node.index.is_node(&addr), false);
            }
        }
    }
    assert_eq!(node.index.is_nodes_empty(), true);
    assert_eq!(node.index.is_nodes_full(), false);
    assert_eq!(node.index.is_runs_empty(), true);
    assert_eq!(node.index.is_runs_full(), false);
}

#[test]
fn node_set_all_l1() {
    // Set all
    let mut node = Node::new(1);
    for k in 0_u8..=255 {
        for b in 0..=63 { 
            let bitno:u64 = (k as u64)*64 + b;
            let addr = Addr::new(bitno);
            node.set(&addr);

            // Up to the last bit of the word, the node 
            // should be marked as set. But after the 
            // 64th bit is set, we clear the nodes vec
            // and set it as a run instead
            if b < 63 {
                assert_eq!(node.index.is_node(&addr), true);
                assert_eq!(node.index.is_run(&addr), false);
            }
            else {
                assert_eq!(node.index.is_node(&addr), false);
                assert_eq!(node.index.is_run(&addr), true);
            }
        }
    }

    // All child nodes should be set, so it should be a run
    println!("ALL SET: index = {:?}:", node.index);
    assert_eq!(node.index.is_nodes_empty(), true);
    assert_eq!(node.index.is_nodes_full(), false);
    assert_eq!(node.index.is_runs_empty(), false);
    assert_eq!(node.index.is_runs_full(), true);

    // Clear first bit - the run bit should be cleared 
    // and first node bit set back to 1
    let addr = Addr::new(0);
    assert_eq!(node.index.is_run(&addr), true);
    assert_eq!(node.index.is_node(&addr), false);

    node.clear(&Addr::new(0));
    println!("FIRST bit cleared: index = {:?}:", node.index);
    assert_eq!(node.index.is_run(&addr), false);
    assert_eq!(node.index.is_node(&addr), true);
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

#[test]
fn iter_bits() {
    // Set bits in the 0 - 256*64 range (raw bits)
    let in_bits = vec![0_u64, 1, 5, 16, 63, 255, 64*255, 64*256-1];
    let mut node = Node::new(1);

    for bitno in &in_bits {
        node.set(&Addr::new(*bitno));
    }
   
    // Iterate and see if we get out the same thing
    let mut out_bits = Vec::new();
    let start_addr = Addr::new(0);
    for bitno in node.iter(start_addr) {
        out_bits.push(bitno);
    }

    assert_eq!(in_bits, out_bits);
}

#[test]
fn iter_run() {
    // Set all bits to form runs for the first 2 keys
    let mut node = Node::new(1);
    let start_bit = 1 * 64;
    let end_bit = 3 * 64 - 1;
    let mut in_bits = Vec::new();

    for b in start_bit..=end_bit {
        node.set(&Addr::new(b));
        in_bits.push(b);
    }

    // Iterate over the runs and compare to our expected
    // input bits
    let mut out_bits = Vec::new();
    for b in node.iter(Addr::new(0)) {
        out_bits.push(b);
    }
    assert_eq!(in_bits, out_bits);
}

#[test]
fn iter_node() {
    let mut node = Node::new(8);
    let in_bits = vec![0_u64, 0xff, 0xff_ff, 0xff_00_01, 
        0xff_00_02, 0xff_ff_ff_ff, u64::MAX];

    for b in &in_bits {
        node.set(&Addr::new(*b));
    }

    let mut out_bits = Vec::new();
    for b in node.iter(Addr::new(0)) {
        out_bits.push(b);
    }
    println!("node is {:?}", node);
    assert_eq!(in_bits, out_bits);
}
