use bitfoo::{BitFoo,MapStore,OffsetStore,BitVecStore};

fn main() {
    let mut map = BitFoo::Map(MapStore::box_new());
    let mut offset = BitFoo::Offset(OffsetStore::box_new());
    let mut bitvec = BitFoo::Vec(BitVecStore::box_new());
    let one = BitFoo::One;
    let zero = BitFoo::Zero;

    println!("map is of type {:?}", map);
    println!("offset is of type {:?}", offset);
    println!("bitvec is of type {:?}", bitvec);
    println!("one is of type {:?}", one);
    println!("zero is of type {:?}", zero);

    println!("-----------------------");
    println!("Setting map bit 3");
    map.set(3);
    println!("Setting offset bit 4");
    offset.set(4);
    println!("Setting bitvec bit 5");
    bitvec.set(5);

    for i in 0..6 {
        println!("-----------------------");
        println!("map[{}] = {}", i, map.get(i));
        println!("offset[{}] = {}", i, offset.get(i));
        println!("bitvec[{}] = {}", i, bitvec.get(i));
        println!("one[{}] = {}", i, one.get(i));
        println!("zero[{}] = {}", i, zero.get(i));
    }

    println!("-----------------------");
    let result = map.and(&offset);
    println!("AND => {:?}", result);

}

