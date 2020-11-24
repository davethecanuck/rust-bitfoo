use bitfoo::{Node,Content,BitFooVec};

fn main() {
    let mut vec = BitFooVec::new();
    println!("Size of BitFooVec is {}-bytes", std::mem::size_of::<BitFooVec>());
    println!("Vec is {:?}", vec);
    vec.set(5);
    vec.set(1024);
    vec.set(1024 * 1024 + 63);
    vec.set(256 * 256 * 256 * 256 * 256 + 63);
}

