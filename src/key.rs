use sbox;
use util;

// Rcon[] is 1-based, so the first entry is just a place holder
const R_CON: [u32; 13*4] = [ 0x00000000, 
    0x01000000, 0x02000000, 0x04000000, 0x08000000, 
    0x10000000, 0x20000000, 0x40000000, 0x80000000, 
    0x1B000000, 0x36000000, 0x6C000000, 0xD8000000, 
    0xAB000000, 0x4D000000, 0x9A000000, 0x2F000000, 
    0x5E000000, 0xBC000000, 0x63000000, 0xC6000000, 
    0x97000000, 0x35000000, 0x6A000000, 0xD4000000, 
    0xB3000000, 0x7D000000, 0xFA000000, 0xEF000000, 
    0xC5000000, 0x91000000, 0x39000000, 0x72000000, 
    0xE4000000, 0xD3000000, 0xBD000000, 0x61000000, 
    0xC2000000, 0x9F000000, 0x25000000, 0x4A000000, 
    0x94000000, 0x33000000, 0x66000000, 0xCC000000, 
    0x83000000, 0x1D000000, 0x3A000000, 0x74000000, 
    0xE8000000, 0xCB000000, 0x8D000000
];

pub struct Key {
    words: Vec<u32>
}

#[derive(Debug, PartialEq)]
pub struct KeySchedule {
    words: Vec<u32>
}

impl Key {
    pub fn new(words: &[u32]) -> Key {
        match words.len() {
            4 => {},
            6 => {},
            8 => {},
            _ => {panic!("Invalid key size!")}
        };

        Key {
            words: words.to_vec()
        }
    }

    pub fn create_schedule(&self) -> KeySchedule {
        let nk = self.words.len();
        let nr = nk + 6;
        let mut vector: Vec<u32> = self.words.clone();

        for i in nk..4*(nr+1) {
            let temp = vector[i-1];
            let temp = if i % nk == 0 {
                sbox::sub_word(util::rot_word(temp)) ^ R_CON[i/nk]
            } else if nk > 6 && i % nk == 4 {
                sbox::sub_word(temp)
            } else {
                temp
            };
            let prev = vector[i-nk];
            vector.push(prev ^ temp);
        }

        KeySchedule::new(&vector)
    }
}

impl KeySchedule {
    pub fn new(words: &[u32]) -> KeySchedule {
        match words.len() {
            44 => {},
            52 => {},
            60 => {},
            _ => {panic!("Invalid key schedule size!")}
        };
        
        KeySchedule {
            words: words.to_vec()
        }
    }
}

#[cfg(test)]
mod tests {
	use key::*;

    #[test]
    fn test_key_schedule() {
        let key = Key::new(&[0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c]);
        let schedule = key.create_schedule();

        let expected = [
            0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c,
            0xa0fafe17, 0x88542cb1, 0x23a33939, 0x2a6c7605,
            0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f,
            0x3d80477d, 0x4716fe3e, 0x1e237e44, 0x6d7a883b,
            0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00,
            0xd4d1c6f8, 0x7c839d87, 0xcaf2b8bc, 0x11f915bc,
            0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
            0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f,
            0xead27321, 0xb58dbad2, 0x312bf560, 0x7f8d292f,
            0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e,
            0xd014f9a8, 0xc9ee2589, 0xe13f0cc8, 0xb6630ca6
        ];

        assert_eq!(KeySchedule::new(&expected), schedule);
    }
}