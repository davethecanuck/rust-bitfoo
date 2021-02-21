#[cfg(test)]
use crate::Addr;

#[test]
fn addr_to_bitno() {
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
        for b in &[0_u64, 1, 0x3f, 0x3f_ff, 0x3f_ff_ff, 
                0x3f_ff_ff_ff, u64::MAX] {  
            // Minimum bitno is where all of the lower bits are 0 below
            // our level. Max is this plus the cardinality of this level.
            let addr = Addr::new(*b);
            let expected_min = (u64::MAX << Addr::offset(level)) & addr.bitno();
            let expected_max = expected_min + Addr::cardinality(level);
            assert_eq!(addr.min_bitno(level), expected_min);
            assert_eq!(addr.max_bitno(level), expected_max);
        }
    }
}

#[test]
fn keys_and_level() {
    let mut first:u64 = 0;
    let mut last:u64 = 0x3f_ff;

    // Level 1 node
    for level in 1_u8..=8 {
        for bitno in &[first, last] {
            // Verify the level
            let addr = Addr::new(*bitno);
            assert_eq!(level, addr.node_level);

            // Verify keys at each level 
            assert_eq!(bitno & 0x3f, addr.key(0) as u64); 
            for key_level in 1..=addr.node_level {
                // 6 bits at level 0, and 8 bits for other levels 
                let key = addr.key(key_level);
                let shift = 6 + 8 * (key_level-1);
                assert_eq!((bitno >> shift) & 0xff, key as u64); 
            }
        }
        
        // Jump to the next boundary bit numbers
        if level < 8 {
            first = last + 1;
            last = (last << 8) | 0xff;
        }
    }
}
