#[cfg(test)]
use crate::Addr;

#[test]
fn keys_and_level() {
    let mut first:u64 = 0;
    let mut last:u64 = 0x3F_FF;

    for level in 1_u8..=8 {
        for bitno in &[first, last] {
            // Verify the level
            let addr = Addr::new(*bitno);
            assert_eq!(level, addr.level());

            // Verify keys at each level 
            assert_eq!(bitno & 0x3F, addr.key(0) as u64); 
            for key_level in 1..=addr.level() {
                // 6 bits at level 0 + 8 bits for other levels 
                let key = addr.key(key_level);
                let shift = 6 + 8 * (key_level-1);
                assert_eq!((bitno >> shift) & 0xFF, key as u64); 
            }
        }
        
        // Jump to the next boundary bit numbers
        if level < 8 {
            first = last + 1;
            last = (last << 8) | 0xFF;
        }
    }
}
