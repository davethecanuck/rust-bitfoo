use std::fmt;

// Constant to get bit shift and mask for each 
// level of the tree
// (bit_offset, mask, max_bit, level)
const LEVEL_PARAM:[(u64, u64, u64, u8);9] = [
    (0, 0x3F, 0x3F, 1),        // 6 bits for within the leaf node
    (6+0*8, 0xFF, 0x3F_FF, 1), // 8 bits for all other levels
    (6+1*8, 0xFF, 0x3F_FF_FF, 2),
    (6+2*8, 0xFF, 0x3F_FF_FF_FF, 3),
    (6+3*8, 0xFF, 0x3F_FF_FF_FF_FF, 4),
    (6+4*8, 0xFF, 0x3F_FF_FF_FF_FF_FF, 5),
    (6+5*8, 0xFF, 0x3F_FF_FF_FF_FF_FF_FF, 6),
    (6+6*8, 0xFF, 0x3F_FF_FF_FF_FF_FF_FF_FF, 7),
    (6+7*8, 0xFF, 0xFF_FF_FF_FF_FF_FF_FF_FF, 8),
];

// Container giving key by level for a u64 bitno
pub struct Addr{
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

            // Set the level of the node that can contain
            // this bitno
            addr.level = param.3;
            if bitno <= param.2 {
                break;
            }
        }
        addr
    }

    pub fn bitno(&self) -> u64 {
        let mut bitno:u64 = 0;
        for i in 0..=self.level {
            let param = LEVEL_PARAM[i as usize];
            bitno += (self.key[i as usize] as u64) << param.0;
        }
        bitno
    }

    pub fn key(&self, level: u8) -> u8 {
        if level <= self.level {
            self.key[level as usize]
        }
        else {
            0_u8
        }
    }

    pub fn set(&mut self, level: u8, key: u8) {
        self.key[level as usize] = key;
    }
}

// Debug interface
impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ level:{}, key:[", self.level)?;
        for level in (0..=self.level).rev() {
            write!(f, " {}:[{:#x}]", level, self.key(level))?;
        }
        write!(f, " ] }}")
    }
}

// Clone interface
impl Clone for Addr {
    fn clone(&self) -> Addr {
        Addr { 
            level: self.level,
            key: self.key,
        }
    }
}

#[cfg(test)]
#[path = "./tests/addr_test.rs"]
mod tests;
