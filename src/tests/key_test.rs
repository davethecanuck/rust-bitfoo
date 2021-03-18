#[cfg(test)]
use crate::{KeyState,KeyIndex,Addr};

#[test]
fn iterator() {
    let mut index = KeyIndex::new(1);
    let input_runs = vec![5_u8, 10, 15, 20];
    let input_nodes = vec![0_u8, 2, 63];

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
fn simple_set() {
    let mut index = KeyIndex::new(1);
    let mut addr = Addr::new(0);
    addr.set(1, 0x3f);
    index.set(&addr);
    println!("addr={:?} index={:?}", addr, index);

    match index.search(&addr) {
        KeyState::Missing(key, offset) => {
            println!("Missing at key={} offset={}", key, offset);
        },
        KeyState::Run(key) => {
            println!("Run at key={}", key);
        },
        KeyState::Node(key, offset) => {
            println!("Node at key={} offset={}", key, offset);
        },
    }

    match index.nodes.offset(0x3f) {
        Ok(offset) => println!("Ok offset={}", offset),
        Err(offset) => println!("Err offset={}", offset),
    }
}

#[test]
fn set_and_search() {
    let level = 1;
    let mut index = KeyIndex::new(level);
    let node_keys = vec![1_u8, 2, 51, 52, 53, 62, 63];
    let run_keys = vec![0_u8, 61];
    let clear_keys = vec![51_u8, 52, 53];

    // Update an Addr struct to set each key at our level 
    let mut addr = Addr::new(0);

    // Set node keys
    println!("=== set NODE ===");
    for key in &node_keys {
        addr.set(level, *key);
        index.set(&addr);
        println!("key={:x} addr={:?}", key, addr);
    }
    println!("index={:?}", index);

    // Set some run keys
    println!("=== set RUN ===");
    for key in &run_keys {
        addr.set(level, *key);
        index.run(&addr);
        println!("key={:x} addr={:?}", key, addr);
    }
    println!("index={:?}", index);

    // Clear some node keys
    println!("=== clear NODE ===");
    for key in &clear_keys {
        addr.set(level, *key);
        index.clear(&addr);
        println!("key={:x} addr={:?}", key, addr);
    }
    println!("index={:?}", index);

    // Check index for all bits at this levels
    for bitno in 0..=Addr::max_bit(level) {
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
