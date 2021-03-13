use std::vec::Vec;
use crate::bitfoo;

#[cfg(test)]
use crate::BitFoo;

#[test]
fn sizeof() {
    let size = std::mem::size_of::<BitFoo>();
    println!("Size of BitFoo is {} bytes", size);
    assert_eq!(size, 104);
}

#[test]
fn set_and_get() {
    let mut bv = BitFoo::new();
    let in_bits = vec![0_u64, 0x3f, 0x40, 0x_3f_ff, 0x40_00, 
        0x3f_ff_ff, 0x40_00_00, 0x3f_ff_ff_ff,  0x40_00_00_00, u64::MAX];

    // Inserting in reverse should result in same output
    for b in in_bits.iter().rev() {
        bv.set(*b);
        println!("iter() SET bit {:x}", b);
    }

    let mut out_bits:Vec<u64> = Vec::new();
    for b in bv.iter() {
        out_bits.push(b);
        println!("iter() GET bit {:x}", b);
    }
    assert_eq!(in_bits, out_bits);

    // Test clone 
    let mut cloned_bits:Vec<u64> = Vec::new();
    let cloned_bv = bv.clone();
    for b in cloned_bv.iter() {
        cloned_bits.push(b);
    }
    assert_eq!(in_bits, cloned_bits);
}

#[test]
fn ops_index() {
    let in_bits = vec![0_u64, 0x3f, 0x40, 0x_3f_ff, 0x40_00];
    let mut bv = BitFoo::new();
    in_bits.iter().for_each(|b| bv.set(*b));

    for i in 0..=0x40_00 {
        assert_eq!(bv[i], in_bits.contains(&i));
    }
}

#[test]
fn macro_new() {
    let bv = bitfoo![0, 1, 0x3f, 0x3fff, 0x4000, u64::MAX];
    let expected = vec![0_u64, 1, 0x3f, 0x3fff, 0x4000, u64::MAX];
    let mut output = Vec::<u64>::new();
    bv.iter().for_each(|b| output.push(b));
    assert_eq!(expected, output);
}