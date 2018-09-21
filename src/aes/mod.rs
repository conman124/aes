use ff::FF;

const S_BOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16
];

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

fn word_to_bytes(word: u32) -> (u8, u8, u8, u8) {
    (
        ((word & 0xff000000) >> 24) as u8,
        ((word & 0x00ff0000) >> 16) as u8,
        ((word & 0x0000ff00) >> 08) as u8,
        ((word & 0x000000ff) >> 00) as u8
    )
}

fn bytes_to_word(bytes: (u8, u8, u8, u8)) -> u32 {
    (bytes.0 as u32) << 24 ^
    (bytes.1 as u32) << 16 ^
    (bytes.2 as u32) << 08 ^
    (bytes.3 as u32)
}

fn sub_word(word: u32) -> u32 {
    let mut bytes = word_to_bytes(word);
    bytes.0 = sub_byte(bytes.0);
    bytes.1 = sub_byte(bytes.1);
    bytes.2 = sub_byte(bytes.2);
    bytes.3 = sub_byte(bytes.3);
    bytes_to_word(bytes)
}

fn sub_byte(byte: u8) -> u8 {
    S_BOX[byte as usize]
}

fn rot_word(word: u32) -> u32 {
    let high: u8 = ((word & 0xff000000) >> 24) as u8;
    let word = word << 8;

    word ^ (high as u32)
}

pub struct Key {
    words: Vec<u32>
}

#[derive(Debug, PartialEq)]
struct KeySchedule {
    words: Vec<u32>
}

#[derive(Debug, PartialEq)]
struct State {
    state: [[u8; 4]; 4]
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

    fn create_schedule(&self) -> KeySchedule {
        let nk = self.words.len();
        let nr = nk + 6;
        let mut vector: Vec<u32> = self.words.clone();

        for i in nk..4*(nr+1) {
            let temp = vector[i-1];
            let temp = if i % nk == 0 {
                sub_word(rot_word(temp)) ^ R_CON[i/nk]
            } else if nk > 6 && i % nk == 4 {
                sub_word(temp)
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

impl State {
    pub fn sub_bytes(&self) -> State {
        let mut ret = [[0;4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                ret[i][j] = sub_byte(self.state[i][j]);
            }
        }

        State{ state: ret }
    }

    pub fn shift_rows(&self) -> State {
        State{state: [
            State::shift_row(&self.state[0], 0),
            State::shift_row(&self.state[1], 1),
            State::shift_row(&self.state[2], 2),
            State::shift_row(&self.state[3], 3),
        ]}
    }

    fn shift_row(row: &[u8; 4], amount: usize) -> [u8; 4] {
        if amount == 0 { return *row; }

        let mut ret = [0; 4];

        for i in 0..4 {
            ret[i] = row[(i+amount) % 4];
        }

        ret
    }

    pub fn mix_columns(&self) -> State {
        let mut ret = self.state.clone();

        for i in 0..4 {
            let col = State::mix_column(&mut ret, i);
            for j in 0..4 {
                ret[j][i] = col[j];
            }
        }

        State{state: ret}
    }

    fn mix_column(arr: &[[u8;4]; 4], col: usize) -> [u8; 4] {
        let mut ret = [0; 4];
        for i in 0..4 {
            ret[i] = (
                  FF::new(arr[(i+0)%4][col]) * FF::new(0x02)
                + FF::new(arr[(i+1)%4][col]) * FF::new(0x03)
                + FF::new(arr[(i+2)%4][col])
                + FF::new(arr[(i+3)%4][col])
            ).value();
        }

        ret
    }

	pub fn add_round_key(&self, slice: &[u32]) -> State {
		let mut ret = [[0;4]; 4];
		for c in 0..4 {
			let word = bytes_to_word((self.state[0][c],self.state[1][c],self.state[2][c],self.state[3][c]));

			let res = word ^ slice[c];
			let bytes = word_to_bytes(res);

			ret[0][c] = bytes.0;
			ret[1][c] = bytes.1;
			ret[2][c] = bytes.2;
			ret[3][c] = bytes.3;
		}

		State{state: ret}
	}
}

#[cfg(test)]
mod test;