use std::time::SystemTime;
use std::{thread, time};
use bitfoo::{Node,Addr};

fn main() {
    println!("Size of instance is {}-bytes", std::mem::size_of::<Node>());

    // Set some bits on a tree of nodes
    let numbits = 1000000;
    let interval = 10;

    // Set initial node to be the same level as the
    // addr of the last bit
    let addr = Addr::new(numbits);
    let mut node = Node::new(addr.level);
    println!("Setting {} bits on {} level node...", 
             numbits, addr.level);

    println!("Start bit SET: {:?}", SystemTime::now());
    for bitno in 0..numbits {
        if bitno % interval == 0 {
            let addr = Addr::new(bitno);
            node.set(&addr);
        }
    }

    // Now go back and check those bits
    // EYE - get() is much slower than set()
    println!("Getting {} bits...", numbits);
    println!("Start bit GET: {:?}", SystemTime::now());
    for bitno in 0..numbits {
        let addr = Addr::new(bitno);
        let _state = node.get(&addr);
    }
    println!("Time FINISHED: {:?}", SystemTime::now());
    thread::sleep(time::Duration::from_secs(1000));
}

