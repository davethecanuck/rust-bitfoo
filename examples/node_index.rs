use bitfoo::{Node,Addr};

fn main() {
    let mut node = Node::new(8);
    for bitno in &[3,7,9] {
        // Set some low bits
        let addr = Addr::new(*bitno);
        println!("Setting bit {}", bitno);
        node.set(&addr);

        // And some high bits
        let bitno = u64::MAX - bitno;
        let addr = Addr::new(bitno);
        println!("Setting bit {}", bitno);
        node.set(&addr);
    }

    for bitno in 0..10 {
        // Check low bits
        let addr = Addr::new(bitno);
        println!("Checking bit [{}]={}", bitno, node[bitno]);
        println!("Checking (bitno={}) addr[{:?}]={}", 
                 bitno, addr, node[&addr]);
    }

    for bitno in u64::MAX-10..=u64::MAX {
        // Check high bits
        println!("Checking bit [{}]={}", bitno, node[bitno]);
        let addr = Addr::new(bitno);
        println!("Checking (bitno={}) addr[{:?}]={}", 
                 bitno, addr, node[&addr]);
    }

    // Clear bit 3
    println!("Before clear: Bit 3 is {}", node[3]);
    let addr = Addr::new(3);
    node.clear(&addr);
    println!("After clear: Bit 3 is {}", node[3]);
}

