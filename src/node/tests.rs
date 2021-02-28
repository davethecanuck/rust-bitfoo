use std::vec::Vec;

#[cfg(test)]
use crate::{Node,Addr};

#[test]
fn simple_set_get() {
    let mut node = Node::new(1);
    for bitno in [0_u64, 2, 63, 64, 127, 128].iter() {
        let addr = Addr::new(*bitno);
        node.set(&addr);
        assert_eq!(node.get(&addr), true);
        println!("Set bitno={:#x}: node={:#?}", bitno, node);
    }
}

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
    assert_eq!(node.index.is_run(&addr), false);
    assert_eq!(node.index.is_node(&addr), true);
}

#[test]
fn set_clear_and_index() {
    for level in 1..=8 {
        // Test some boundary bits in each node level
        let mut node = Node::new(level);
        let input_bits = vec![0_u64, 1,  
            Addr::child_cardinality(level), 
            Addr::child_cardinality(level) + 1, 
            Addr::max_bit(level)];
        println!("set_clear_and_index: level={} child_cardinality={:x} max_bit={:x}", 
            level, Addr::child_cardinality(level), Addr::max_bit(level));
        println!("    node={:?}", node);

        for bitno in input_bits {
            println!("Testing bit={:x}", bitno);
            assert_eq!(false, node[bitno]);
            node.set(&Addr::new(bitno));
            assert_eq!(true, node[bitno]);
            node.clear(&Addr::new(bitno));
            assert_eq!(false, node[bitno]);
        }
    }
}

#[test]
fn iter_bits() {
    // Set bits in the 0 - 256*64 range (raw bits)
    let in_bits = vec![0_u64, 1, 0x3f, 0x40, 0x3fff];
    let mut node = Node::new(1);
    println!("iter_bits: in_bits={:?}", in_bits);

    for bitno in &in_bits {
        let addr = Addr::new(*bitno);
        println!("    iter_bits  SET1: bitno={:x} addr={:?}", bitno, addr);
        node.set(&addr);
        println!("    iter_bits  SET2: node={:?}", node);
    }
   
    // Iterate and see if we get out the same thing
    let mut out_bits = Vec::new();
    for bitno in node.iter() {
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
    for b in node.iter() {
        out_bits.push(b);
    }
    assert_eq!(in_bits, out_bits);
}

#[test]
fn iter_node() {
    let mut node = Node::new(8);
    let in_bits = vec![0_u64, 0x3f, 0x40, 0x3f_ff, 0x40_00, 
        0x3f_ff_ff, 0x40_00_00, u64::MAX];

    for b in &in_bits {
        node.set(&Addr::new(*b));
    }

    let mut out_bits = Vec::new();
    for b in node.iter() {
        out_bits.push(b);
    }
    //println!("node is {:?}", node);
    assert_eq!(in_bits, out_bits);
}
