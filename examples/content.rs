use bitfoo::{Content,NodeVec};

fn main() {
    let c1 = Content::Child(NodeVec::new());     
    let c2 = Content::Bits(3);     
    let c3 = Content::Ones;

    println!("c1 is {:?}", c1);
    println!("c2 is {:?}", c2);
    println!("c3 is {:?}", c3);
}

