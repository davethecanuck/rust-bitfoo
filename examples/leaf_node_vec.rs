use bitfoo::{Addr,LeafNode,LeafNodeVec};

fn main() {
    println!("Size of LeafNodeVec is {}-bytes", std::mem::size_of::<LeafNodeVec>());
    let mut vec = LeafNodeVec::new();

    // Set some bits
    for bitno in 0..100 {
        if bitno % 10 == 0 {
            // As a leaf node, the level will be 1
            let addr = Addr::new(bitno);
            vec.set(&addr);
        }
    }

    // Now go back and check those bits
    for bitno in 0..100 {
        let addr = Addr::new(bitno);
        let state = vec.get(&addr);
        println!("{} is {}", bitno, state);
    }
                   
    /*
    for bitno in 0..100 {
        println!("Searching for key={}", key);
        match nv.search(key) {
            Ok((offset, node)) => {
                println!("  FOUND: offset={} node={:?}", offset, node);
            }
            Err(offset) => {
                println!("  MISSING: offset={}", offset);
            }
        }
    }
    */
}

