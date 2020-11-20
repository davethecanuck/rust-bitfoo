// Constant to get bit shift and mask for each 
// level of the tree
// (bit_offset, mask, max_bitno)
const LEVEL_PARAM:[(u64,u64);9] = [
    (0, 0x3F),     // 6 bits for level 0
    (6, 0xFF),     // 8 bits for all other levels
    (6+8, 0xFF),
    (6+2*8, 0xFF),
    (6+3*8, 0xFF),
    (6+4*8, 0xFF),
    (6+5*8, 0xFF),
    (6+6*8, 0xFF),
    (6+7*8, 0xFF)
];

// Container giving offsets by level for a u64 bitno
 #[derive(Debug)]
pub struct Addr {
    level: u8,
    offset: [u8;9],
}

impl Addr {
    pub fn new(bitno: u64) -> Self {
        let mut addr = Addr {
            level: 0,
            offset: [0;9],
        };

        for i in 0..addr.offset.len() {
            let param = LEVEL_PARAM[i as usize];
            addr.offset[i] = ((bitno >> param.0) & param.1) as u8;
            if addr.offset[i] > 0 {
                addr.level = i as u8;
            }
        }
        addr
    }

    pub fn offset(&self, level: u8) -> u8 {
        if (level) <= self.level {
            self.offset[level as usize]
        }
        else {
            0_u8
        }
    }
}

