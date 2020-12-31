#[cfg(test)]
use crate::BitFooVec;

#[test]
fn sizeof() {
    let size = std::mem::size_of::<BitFooVec>();
    println!("Size of BitFooVec is {} bytes", size);
    assert_eq!(size, 104);
}

#[test]
fn set_and_get() {
    let max = 16384; //_000_000;
    let mut bv = BitFooVec::new();
    let bits = vec![0_u64, 100, 255, 500, 16000, max];

    for b in &bits {
        bv.set(*b);
    }

    for b in 0_u64..=max {
        assert_eq!(bv.get(b), bits.contains(&b));
    }
}
