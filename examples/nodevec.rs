use bitfoo::{Node,NodeVec,Content};

fn main() {
    println!("Size of NodeVec is {}-bytes", std::mem::size_of::<NodeVec>());
    let n1 = Node::new(4, Content::Ones);
    let n2 = Node::new(7, Content::Ones);

    let mut nv = NodeVec::new();
    println!("Initial: capacity={} size_of_val={}", nv.capacity(), 
             std::mem::size_of_val(&nv));

    nv.push(n1);
    println!("Add n1: capacity={} size_of_val={}", nv.capacity(),
             std::mem::size_of_val(&nv));

    nv.push(n2);
    println!("Add n2: capacity={} size_of_val={}", nv.capacity(),
             std::mem::size_of_val(&nv));

    for key in 0_u8..10 {
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
}

