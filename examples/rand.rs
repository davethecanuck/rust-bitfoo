use rand::Rng;
use std::time::SystemTime;
use std::{thread, time};
use bitfoo::{Node,Addr};

fn main() {
    println!("Size of instance is {}-bytes", std::mem::size_of::<Node>());

    // Set some bits on a tree of nodes
    let numbits = 1000000;
    let interval = 10;

    // Set to the max level as bits could be any u64
    let mut node = Node::new(8);
    println!("Setting {} bits on {} level node...", 
             numbits, node.level);

    // Randomly set the required number of bits
    let mut rng = rand::thread_rng();
    
    println!("Start bit SET: {:?}", SystemTime::now());
    for _n in 0..numbits {
        let bitno: u64 = rng.gen();
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
    thread::sleep(time::Duration::from_secs(1000));
}

