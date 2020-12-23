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
