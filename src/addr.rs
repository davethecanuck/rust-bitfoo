// Constant to get bit shift and mask for each 
// level of the tree
// (bit_offset, mask)
const LEVEL_PARAM:[(u64,u64);9] = [
    (0, 0x3F),     // 6 bits for within the level 0 node
    (6+0*8, 0xFF), // 8 bits for all other levels
    (6+1*8, 0xFF),
    (6+2*8, 0xFF),
    (6+3*8, 0xFF),
    (6+4*8, 0xFF),
    (6+5*8, 0xFF),
    (6+6*8, 0xFF),
    (6+7*8, 0xFF)
];

// Container giving key by level for a u64 bitno
#[derive(Debug)]
pub struct Addr {
    pub level: u8,
    key: [u8;9],
}

impl Addr {
    pub fn new(bitno: u64) -> Self {
        let mut addr = Addr {
            level: 1,
            key: [0;9],
        };

        for i in 0..addr.key.len() {
            let param = LEVEL_PARAM[i as usize];
            addr.key[i] = ((bitno >> param.0) & param.1) as u8;
            if addr.key[i] > 0 {
                // Always at least 1 level
                addr.level = (std::cmp::max(i, 1)) as u8;
            }
        }
        addr
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn key(&self, level: u8) -> u8 {
        if (level) <= self.level {
            self.key[level as usize]
        }
        else {
            0_u8
        }
    }
}

#[cfg(test)]
#[path = "./addr_test.rs"]
mod tests;
