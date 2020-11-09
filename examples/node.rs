use bitfoo::Node;

fn main() {
    let mut node = Node::new(0);
    println!("node is of type {:?}", node.store);
    println!("Setting node bit 4");
    node.store.set(4);

    for i in 0..6 {
        println!("-----------------------");
        println!("node[{}] = {}", i, node.store.get(i));
    }
}

