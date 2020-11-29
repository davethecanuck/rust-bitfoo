use bitfoo::{Addr,LeafVec};

fn main() {
    let mut vec = LeafVec::new();
    println!("Size of instance is {}-bytes", std::mem::size_of_val(&vec));

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
                   
    println!("Size of LeafVec is {}-bytes", std::mem::size_of::<LeafVec>());
    println!("Size of instance is {}-bytes", std::mem::size_of_val(&vec));
    // EYE - We only get the size of the main struct, but not that of 
    // it's vectors on the heap
}

