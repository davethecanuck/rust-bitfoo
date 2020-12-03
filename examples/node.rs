use std::time::SystemTime;
use bitfoo::{Node,Addr};

fn main() {
    println!("Size of instance is {}-bytes", std::mem::size_of::<Node>());

    // Set some bits on a tree of nodes
    let numbits = 1000000;
    let interval = 100;

    // Set initial node to be the same level as the
    // addr of the last bi
    let addr = Addr::new(numbits);
    let mut node = Node::new(addr.level());
    println!("Setting {} bits on {} level node...", 
             numbits, node.level);

    println!("Start bit SET: {:?}", SystemTime::now());
    for bitno in 0..numbits {
        if bitno % interval == 0 {
            let addr = Addr::new(bitno);
            println!("Setting bitno={} addr={:?}", bitno, addr);
            node.set(&addr);
        }
    }

    // Now go back and check those bits
    // EYE - get() is much slower than set()
    println!("Getting {} bits...", numbits);
    println!("Start bit GET: {:?}", SystemTime::now());
    for bitno in 0..numbits {
        let addr = Addr::new(bitno);
        let state = node.get(&addr);
        println!("{} is {}", bitno, state);
    }
    println!("Time FINISHED: {:?}", SystemTime::now());
}

