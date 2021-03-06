#[cfg(test)]
use crate::{KeyState,KeyIndex,Addr};

#[test]
fn iterator() {
    let mut index = KeyIndex::new(1);
    let input_runs = vec![5_u8, 10, 15, 20];
    let input_nodes = vec![0_u8, 2, 128, 255];

    for key in &input_runs {
        // 64 bits at each key for level 1 index
        let addr = Addr::new(*key as u64 * 64);
        index.run(&addr);
    }

    for key in &input_nodes {
        // 64 bits at each key for level 1 index
        let addr = Addr::new(*key as u64 * 64);
        index.set(&addr);
    }

    // Iterate through and compare to our initial inputs
    let mut output_runs = Vec::<u8>::new();
    let mut output_nodes = Vec::<u8>::new();
    let mut curr_offset = 0;

    for key_state in index.iter() {
        match key_state {
            KeyState::Node(key, offset) => {
                output_nodes.push(key);
                assert_eq!(offset, curr_offset);
                curr_offset += 1;
            },
            KeyState::Run(key) => {
                output_runs.push(key);
            },
            _ => ()
        }
    }

    assert_eq!(input_runs, output_runs);
    assert_eq!(input_nodes, output_nodes);
}
    
#[test]
fn set_and_search() {
    // Create level 1 index for keys 0-255
    let mut index = KeyIndex::new(1);
    let node_keys = vec![0_u8, 50, 50, 100, 150, 250, 255];
    let run_keys = vec![1_u8, 100, 200];
    let clear_keys = vec![200_u8, 250, 251];

    // Set node, run and clear keys
    for key in &node_keys {
        // 64 bits at each key for level 1 index
        let addr = Addr::new(*key as u64 * 64);
        index.set(&addr);
    }

    // Set some runs 
    for key in &run_keys {
        let addr = Addr::new(*key as u64 * 64);
        index.run(&addr);
    }

    // Clear a node
    for key in &clear_keys {
        let addr = Addr::new(*key as u64 * 64);
        index.clear(&addr);
    }

    // Check states 
    for bitno in 0..=u8::MAX as u64 * 64 {
        let addr = Addr::new(bitno as u64);
        let key = index.key(&addr);
        let is_correct:bool;

        // We will compare our search output to what is 
        // in the input vectors
        let clear = clear_keys.contains(&key);
        let node = node_keys.contains(&key);
        let run = run_keys.contains(&key);

        match index.search(&addr) {
            KeyState::Run(_key) => {
                is_correct = run && !clear;
            },
            KeyState::Node(_key, _offset) => {
                is_correct = node && !run && !clear;
            },
            KeyState::Missing(_key, _offset) => {
                is_correct = clear || !(run || node);
            },
        }
        assert_eq!(is_correct, true);
    }
}
