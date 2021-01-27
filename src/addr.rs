use std::fmt;

// Constant to get bit shift and mask for each 
// level of the tree
// (bit_offset, mask, max_bit, node_level)
const LEVEL_PARAM:[(u64, u64, u64, u8);10] = [
    (0,     0x3f, 0x3f, 1),        // 6 bits for within the leaf node
    (6+0*8, 0xff, 0x3f_ff, 1),     // 8 bits for all other levels
    (6+1*8, 0xff, 0x3f_ff_ff, 2),
    (6+2*8, 0xff, 0x3f_ff_ff_ff, 3),
    (6+3*8, 0xff, 0x3f_ff_ff_ff_ff, 4),
    (6+4*8, 0xff, 0x3f_ff_ff_ff_ff_ff, 5),
    (6+5*8, 0xff, 0x3f_ff_ff_ff_ff_ff_ff, 6),
    (6+6*8, 0xff, 0x3f_ff_ff_ff_ff_ff_ff_ff, 7),
    (6+7*8, 0xff, 0xff_ff_ff_ff_ff_ff_ff_ff, 8),
    (8*8, 0xff, 0xff_ff_ff_ff_ff_ff_ff_ff, 9), // One node at top level
];

// Container giving key by level for a u64 bitno
pub struct Addr{
    pub node_level: u8,
    key: [u8;10],
}

// Class level functions
impl Addr {
    // Constructor
    pub fn new(bitno: u64) -> Self {
        let mut addr = Addr {
            node_level: 1,
            key: [0;10],
        };

        for i in 0..addr.key.len() {
            let param = LEVEL_PARAM[i as usize];
            addr.key[i] = ((bitno >> param.0) & param.1) as u8;

            // Set the level of the node that can contain
            // this bitno
            addr.node_level = param.3;
            if bitno <= param.2 {
                break;
            }
        }
        addr
    }

    // Return cardinality for this node level
    pub fn cardinality(level: u8) -> u64 {
        match level {
            // Cardinality is from the level below
            1..=9 => LEVEL_PARAM[(level-1) as usize].2,
            _ => 0,
        }
    }

    // Return bit offset (from u64) for this node level
    pub fn offset(level: u8) -> u64 {
        match level {
            1..=9 => LEVEL_PARAM[level as usize].0,
            _ => 0,
        }
    }
}

// Public methods
impl Addr {
    // Convert Addr to bit numberr
    pub fn bitno(&self) -> u64 {
        let mut bitno:u64 = 0;
        for i in 0..=self.node_level {
            bitno += (self.key[i as usize] as u64) << Addr::offset(i);
        }
        bitno
    }

    // Return the lowest bitno for our address at the given level
    pub fn min_bitno(&self, level: u8) -> u64 {
        // EYE - validate the shift - perhaps should be the next level?
        let mask = u64::MAX << Addr::offset(level);
        self.bitno() & mask

        /* EYE the above should be equivalent
        let mut addr = self.clone();
        for lev in 0..level {  // Set everything below the level to 0
            addr.set(lev, 0);
        }
        addr.bitno()
        */
    }

    // Return the highest bitno for our address at the given level
    pub fn max_bitno(&self, level: u8) -> u64 {
        self.min_bitno(level) + Addr::cardinality(level)
    }

    pub fn key(&self, level: u8) -> u8 {
        if level <= self.node_level {
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