use bitfoo::{Node,Content,BitFooVec};

fn main() {
    let mut vec = BitFooVec::new();
    println!("Vec is {:?}", vec);
    vec.set(5);
    vec.set(1024);
    vec.set(1024 * 1024 + 63);
    vec.set(256 * 256 * 256 * 256 * 256 + 63);
}

