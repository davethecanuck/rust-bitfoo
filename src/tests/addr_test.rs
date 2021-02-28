#[cfg(test)]
use crate::Addr;

#[test]
fn bitno_to_addr() {
    for bitno in vec![0, 1, 0x3f_ff, 0x3f_ff_ff, u64::MAX] {
        let addr = Addr::new(bitno);
        assert_eq!(bitno, addr.bitno());
    }
}

#[test]
fn clone() {
    let input = vec![0_u64, 100000, u64::MAX];
    let mut output = Vec::<u64>::new();

    for b in &input {
        let addr = Addr::new(*b);
        output.push(addr.clone().bitno());
    }

    for i in 0..output.len() {
        assert_eq!(output[i], input[i]);
    }
}

#[test]
fn min_max() {
    for level in 1_u8..=8 {
        for b in &[0_u64, 1, 0x3f, 0x40, 0x3f_fe, 0x3f_ff, 
                0x3f_ff_ff, 0x3f_ff_ff_ff, u64::MAX] {  
            // Minimum bitno is where all of the lower bits are 0 below
            // our level. Max is this plus the cardinality of this level.
            let addr = Addr::new(*b);
            let expected_min = (u64::MAX << Addr::offset(level)) & addr.bitno();
            let expected_max = expected_min + Addr::child_cardinality(level);
            assert_eq!(addr.min_bitno(level), expected_min);
            assert_eq!(addr.max_bitno(level), expected_max);
        }
    }
}

#[test]
fn keys_and_level() {
    // Level 1 addresses
    for level in 0_u8..=8 {
        let min_bitno = Addr::child_cardinality(level) + 1;
        check_key_and_level(level, min_bitno);
        let max_bitno = Addr::max_bit(level);
        check_key_and_level(level, max_bitno);
    }
}

// Helper for keys_and_level
fn check_key_and_level(level:u8, bitno: u64) {
    let addr = Addr::new(bitno);
    assert_eq!(addr.node_level, level);
    assert_eq!(addr.key(level), 
        (bitno >> Addr::offset(level) & Addr::mask(level)) as u8);
}
