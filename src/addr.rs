use std::fmt;

// Constant to get bit shift and mask for each 
// level of the tree
// (bit_offset, mask, max_bit, node_level)
const LMASK: u64 = 0b_00_11_1111;
const LBITS: u64 = 6;

const LEVEL_PARAM:[(u64, u64, u64, u8);11] = [
    (0 *  LBITS, LMASK, u64::MAX >> (64 - 1 * LBITS), 0),
    (1 *  LBITS, LMASK, u64::MAX >> (64 - 2 * LBITS), 1),
    (2 *  LBITS, LMASK, u64::MAX >> (64 - 3 * LBITS), 2),
    (3 *  LBITS, LMASK, u64::MAX >> (64 - 4 * LBITS), 3),
    (4 *  LBITS, LMASK, u64::MAX >> (64 - 5 * LBITS), 4),
    (5 *  LBITS, LMASK, u64::MAX >> (64 - 6 * LBITS), 5),
    (6 *  LBITS, LMASK, u64::MAX >> (64 - 7 * LBITS), 6),
    (7 *  LBITS, LMASK, u64::MAX >> (64 - 8 * LBITS), 7),
    (8 *  LBITS, LMASK, u64::MAX >> (64 - 9 * LBITS), 8),
    (9 *  LBITS, LMASK, u64::MAX >> (64 - 10 * LBITS), 9), // 10 * LBITS = 60
    (10 * LBITS, LMASK, u64::MAX, 10),
];
pub const MAX_LEVEL:u8 = (LEVEL_PARAM.len() - 1) as u8;

// Container giving key by level for a u64 bitno
pub struct Addr{
    pub node_level: u8,
    pub key: [u8;LEVEL_PARAM.len()],
}

// Public class level functions
impl Addr {
    // Constructor
    pub fn new(bitno: u64) -> Self {
        let mut addr = Addr {
            node_level: 0,
            key: [0;LEVEL_PARAM.len()],
        };

        for i in 0..addr.key.len() {
            let param = LEVEL_PARAM[i as usize];
            addr.key[i] = ((bitno >> param.0) & param.1) as u8;

            // Set the level of the node that can contain this bitno
            addr.node_level = param.3;
            if bitno <= param.2 {
                break;
            }
        }
        addr
    }

    // Return the bit mask for this level
    pub fn mask(level: u8) -> u64 {
        match level {
            0..=MAX_LEVEL => LEVEL_PARAM[level as usize].1,
            _ => 0,
        }
    }

    // Return the max bit number for a node at this level
    pub fn max_bit(level: u8) -> u64 {
        match level {
            0..=MAX_LEVEL => LEVEL_PARAM[level as usize].2,
            _ => 0,
        }
    }

    // Return the cardinality of a child node from this level
    pub fn child_max_bit(level: u8) -> u64 {
        match level {
            1..=MAX_LEVEL => LEVEL_PARAM[(level-1) as usize].2,
            _ => 0,
        }
    }

    // Return bit offset (from u64) for this node level
    pub fn offset(level: u8) -> u64 {
        match level {
            0..=MAX_LEVEL => LEVEL_PARAM[level as usize].0,
            _ => 0,
        }
    }
}

// Public methods
impl Addr {
    // Convert Addr to bit numberr
    pub fn bitno(&self) -> u64 {
        let mut bitno:u64 = 0;
        for level in 0..=self.node_level {
            bitno += (self.key[level as usize] as u64) << Addr::offset(level);
        }
        bitno
    }

    // Return the lowest bitno for our address at the given level
    pub fn min_bitno(&self, level: u8) -> u64 {
        match level {
            0..=MAX_LEVEL => {
                let mask = u64::MAX << Addr::offset(level);
                self.bitno() & mask
            },
            _ => 0,
        }
    }

    // Return the highest bitno for our address at the given level
    pub fn max_bitno(&self, level: u8) -> u64 {
        match level {
            1..=MAX_LEVEL => {
                self.min_bitno(level) + Addr::child_max_bit(level)
            },
            _ => 0,
        }
    }

    pub fn key(&self, level: u8) -> u8 {
        match level {
            0..=MAX_LEVEL => self.key[level as usize],
            _ => 0,
        }
    }

    pub fn set(&mut self, level: u8, key: u8) {
        match level {
            0..=MAX_LEVEL => {
                self.key[level as usize] = key;
                if level > self.node_level {
                    self.node_level = level;
                }
            },
            _ => (),
        }
    }
}

// Debug interface
impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ level:{}, key:[", self.node_level)?;
        for level in (0..=self.node_level).rev() {
            write!(f, " {}:[{:#x}]", level, self.key(level))?;
        }
        write!(f, " ] }}")
    }
}

// Clone interface
impl Clone for Addr {
    fn clone(&self) -> Addr {
        Addr { 
            node_level: self.node_level,
            key: self.key,
        }
    }
}

#[cfg(test)]
#[path = "./tests/addr_test.rs"]
mod tests;