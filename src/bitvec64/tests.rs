#[cfg(test)]
use crate::bitvec64;
use crate::BitVec64;

#[test]
fn clone() {
    let mut v = BitVec64(0);
    v.set(0);
    v.set(1);
    let mut v2 = v.clone();
    v.clear(0);
    v2.set(2); 

    // Only the first byte of word 0 
    // has bits set
    assert_eq!(v.raw_data(), 0b_0000_0010);
    assert_eq!(v2.raw_data(), 0b_0000_0111);
}

#[test]
fn simple_set_and_clear() {
    let mut v = BitVec64(0);

    for bitno in [0, 2, 4, 6].iter() {
        v.set(*bitno);
    }
    assert_eq!(v.raw_data(), 0b_0101_0101);

    // Clear some bits
    v.clear(0);
    v.clear(6);
    assert_eq!(v.raw_data(), 0b_0001_0100);
}

#[test]
fn index() {
    let mut v = BitVec64(0);
    assert_eq!(v[0], false);
    v.set(0);
    assert_eq!(v[0], true);
    assert_eq!(v[1], false);
    v.set(1);
    assert_eq!(v[1], true);
    assert_eq!(v[2], false);
    v.set(2);
    assert_eq!(v[2], true);
    v.clear(2);
    assert_eq!(v[2], false);
    assert_eq!(v[bitvec64::MAX_BIT], false);
    v.set(bitvec64::MAX_BIT);
    assert_eq!(v[bitvec64::MAX_BIT], true);
}

#[test]
fn iterator() {
    // Populate a vector
    let mut v = BitVec64(0);
    let input_bits = vec![0_u8, 28, bitvec64::MAX_BIT];
    for i in 0..input_bits.len() {
        v.set(input_bits[i]);
        println!("set - {}", input_bits[i]);
    }

    // Now iterate through the vector and 
    // check that it matches the input
    let mut output_bits = Vec::new();
    for b in v.iter() {
        output_bits.push(b);
    }
    assert_eq!(&input_bits, &output_bits);
}

#[test]
fn iter_all() {
    // Set and iterate through all bits
    let mut v = BitVec64(0);
    for bitno in 0..=bitvec64::MAX_BIT {
        assert_eq!(v.get(bitno), false);
        v.set(bitno);
        assert_eq!(v.get(bitno), true);
    }

    for bitno in v.iter() {
        assert_eq!(v.get(bitno), true);
        match v.offset(bitno) {
            // Bit is set for every offset - they'll match
            Ok(offset)  => {
                assert_eq!(bitno, offset);
            },
            Err(offset) => {
                panic!("bitno={} is missing. offset should be {}", bitno, offset);
            }
        }
    }
}

#[test]
fn get_set_and_clear() {
    let mut v = BitVec64(0);
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
    let mut v = BitVec64(0);
    assert_eq!(v.is_full(), false);
    assert_eq!(v.is_empty(), true);

    // Now fill the vector 
    for bitno in 0..=bitvec64::MAX_BIT {
        assert_eq!(v.is_full(), false);
        v.set(bitno);
        assert_eq!(v.is_empty(), false);
    }
    assert_eq!(v.is_full(), true);
    assert_eq!(v.is_empty(), false);
}

#[test]
fn bitand() {
    let mut a = BitVec64(0);
    let mut b = BitVec64(0);
    a.set(0);
    a.set(1);
    b.set(1);
    let c = a & b;
    assert_eq!(c[0], false); 
    assert_eq!(c[1], true); 
    assert_eq!(c.raw_data(), 0b_0010); 
}

#[test]
fn bitor() {
    let mut a = BitVec64(0);
    let mut b = BitVec64(0);
    a.set(0);
    b.set(0);
    b.set(1);
    let c = a | b;
    assert_eq!(c[0], true); 
    assert_eq!(c[1], true); 
    assert_eq!(c.raw_data(), 0b_0011); 
}

#[test]
fn offset() {
    let mut bv = BitVec64(0);
    //let bitno = [0_u8, 5, 32, bitvec64::MAX_BIT];
    let bitno = [bitvec64::MAX_BIT];
    for b in &bitno {
        bv.set(*b);
    }
    println!("bitvec64.offset() vec={:?}", bv);

    // Step through the input bitno array (in reverse for fun)
    // and verify that the offset in the input array matches the
    // offset calculated in the BitVec64
    for i in (0..bitno.len()).rev() {
        let b = bitno[i];
        match bv.offset(b) {
            Ok(off) => assert_eq!(off as usize, i),
            Err(_off) => panic!("Bitno {} not found", b)
        }
    }
}