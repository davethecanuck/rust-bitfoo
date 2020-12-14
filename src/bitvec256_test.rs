#[cfg(test)]
use crate::BitVec256;

#[test]
fn location() {
    let v = BitVec256::new();
    assert_eq!(v.location(63), (0,63));
    assert_eq!(v.location(249), (3,57));
    assert_eq!(v.location(255), (3,63));
}

#[test]
fn clone() {
    let mut v = BitVec256::new();
    v.set(0);
    v.set(1);
    let mut v2 = v.clone();
    v.clear(0);
    v2.set(2); 

    // Only the first byte of word 0 
    // has bits set
    assert_eq!(v.raw_data(0),  0b_0000_0010);
    assert_eq!(v2.raw_data(0), 0b_0000_0111);
}

#[test]
fn raw_data() {
    let mut v = BitVec256::new();

    // Set bits amongst each of the words
    for bitno in [0,65,130,254,255].iter() {
        v.set(*bitno);
    }
    assert_eq!(v.raw_data(0), 0b_0000_0001);
    assert_eq!(v.raw_data(1), 0b_0000_0010);
    assert_eq!(v.raw_data(2), 0b_0000_0100);
    assert_eq!(v.raw_data(3), 0b_1100_0000 << 7*8);

    // Clear some bits
    v.clear(0);
    v.clear(255);
    assert_eq!(v.raw_data(0), 0b_0000_0000);
    assert_eq!(v.raw_data(3), 0b_0100_0000 << 7*8);
}

#[test]
fn index() {
    let mut v = BitVec256::new();
    assert_eq!(false, v[0]);
    v.set(0);
    assert_eq!(true, v[0]);
    v.set(1);
    assert_eq!(true, v[1]);
    v.set(2);
    assert_eq!(true, v[2]);
    v.clear(2);
    assert_eq!(false, v[2]);
    v.set(255);
    assert_eq!(true, v[255]);
}

#[test]
fn iterator() {
    // Populate a vector
    let mut v = BitVec256::new();
    let input_bits = vec![0_u8, 28, 65, 129, 255];
    for i in 0..input_bits.len() {
        v.set(input_bits[i]);
    }

    // Now iterate throuh the vector and 
    // check that it matches the input
    let mut output_bits = Vec::new();
    for b in v.iter() {
        output_bits.push(b);
    }
    assert_eq!(&input_bits, &output_bits);
}

#[test]
fn get_set_and_clear() {
    let mut v = BitVec256::new();
    assert_eq!(true, v.is_empty());

    let max_bit = 13;
    let set_bit = 5;
    let clear_bit = 10;

    // Set a bunch of bits and then test 
    // after all are set
    for bitno in 0..=max_bit {
        // Should start off all 0/false
        assert_eq!(v.get(bitno), false);
        
        if bitno % set_bit == 0 {
            v.set(bitno);
            if bitno % clear_bit == 0 {
                v.clear(bitno);
            }
        }
    }
    assert_eq!(false, v.is_empty());

    for bitno in 0..=max_bit {
        if bitno % set_bit == 0 {
            if bitno % clear_bit == 0 {
                assert_eq!(false, v.get(bitno));
            }
            else {
                assert_eq!(true, v.get(bitno));
            }
        }
        else {
            // All others were not set
            assert_eq!(false, v.get(bitno));
        }
    }
}

#[test]
fn empty_and_full() {
    let mut v = BitVec256::new();
    assert_eq!(true, v.is_empty());
    assert_eq!(false, v.is_full());

    // Now fill the vector 
    for bitno in 0..=u8::MAX {
        assert_eq!(false, v.is_full());
        v.set(bitno);
        assert_eq!(false, v.is_empty());
    }
    assert_eq!(false, v.is_empty());
    assert_eq!(true, v.is_full());
}

#[test]
fn bitand() {
    let mut a = BitVec256::new();
    let mut b = BitVec256::new();
    a.set(0);
    a.set(1);
    b.set(1);
    let c = a & b;
    assert_eq!(false, c[0]); 
    assert_eq!(true, c[1]); 
    assert_eq!(0b_0010, c.data[0]); 
}

#[test]
fn bitor() {
    let mut a = BitVec256::new();
    let mut b = BitVec256::new();
    a.set(0);
    b.set(0);
    b.set(1);
    let c = a | b;
    assert_eq!(true, c[0]); 
    assert_eq!(true, c[1]); 
    assert_eq!(0b_0011, c.data[0]); 
}

#[test]
fn offset() {
    let mut a = BitVec256::new();
    a.set(0);
    a.set(5);
    a.set(68);
    a.set(129);
    a.set(200);
    a.set(255);

    // EYE Need a full test including Err and Ok
    assert_eq!(0, a.offset(0).unwrap());
}

