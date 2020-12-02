use bitfoo::{Node,Addr};

fn main() {
    println!("Size of instance is {}-bytes", std::mem::size_of::<Node>());
    let mut node = Node::new(1);

    // Set some bits
    for bitno in 0..100 {
        if bitno % 10 == 0 {
            let addr = Addr::new(bitno);
            node.set(&addr);
        }
    }

    // Now go back and check those bits
    for bitno in 0..100 {
        let addr = Addr::new(bitno);
        let state = node.get(&addr);
        println!("{} is {}", bitno, state);
    }
}

