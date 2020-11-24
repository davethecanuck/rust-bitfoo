use bitfoo::{Node,Content};

fn main() {
    let node = Node::new(0, Content::Ones);
    println!("node is {:?}", node);
    println!("Size of Node is {}-bytes", std::mem::size_of::<Node>());
}

