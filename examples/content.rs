use bitfoo::{Content,NodeVec};

fn main() {
    let c1 = Content::Child(NodeVec::new());     
    println!("c1 is {:?}, size={}-bytes", c1, std::mem::size_of_val(&c1));

    let c2 = Content::Bits(3);     
    println!("c2 is {:?}, size={}-bytes", c2, std::mem::size_of_val(&c2));

    let c3 = Content::Ones;
    println!("c3 is {:?}, size={}-bytes", c3, std::mem::size_of_val(&c3));

    println!("Size of Content is {}-bytes", 
             std::mem::size_of::<Content>());
}

