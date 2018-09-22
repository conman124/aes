use std::fmt;
use std::result;

use ff::FF;
use sbox;
use util;

#[derive(Debug, PartialEq)]
pub struct State {
    state: [[u8; 4]; 4]
}

impl State {
	pub fn from_slice(slice: &[u8]) -> State {
		let mut state = [[0; 4]; 4];

		for c in 0..4 {
			for r in 0..4 {
				state[r][c] = slice[c*4 + r];
			}
		}

		State{state}
	}

    pub fn sub_bytes(&self) -> State {
        let mut ret = [[0;4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                ret[i][j] = sbox::sub_byte(self.state[i][j]);
            }
        }

        State{ state: ret }
    }

    pub fn inv_sub_bytes(&self) -> State {
        let mut ret = [[0;4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                ret[i][j] = sbox::inv_sub_byte(self.state[i][j]);
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

    pub fn inv_shift_rows(&self) -> State {
        State{state: [
            State::inv_shift_row(&self.state[0], 0),
            State::inv_shift_row(&self.state[1], 1),
            State::inv_shift_row(&self.state[2], 2),
            State::inv_shift_row(&self.state[3], 3),
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

    fn inv_shift_row(row: &[u8; 4], amount: usize) -> [u8; 4] {
        if amount == 0 { return *row; }

        let mut ret = [0; 4];

        for i in 0..4 {
            ret[i] = row[(i+4-amount) % 4];
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

    pub fn inv_mix_columns(&self) -> State {
        let mut ret = self.state.clone();

        for i in 0..4 {
            let col = State::inv_mix_column(&mut ret, i);
            for j in 0..4 {
                ret[j][i] = col[j];
            }
        }

        State{state: ret}
    }

    fn inv_mix_column(arr: &[[u8;4]; 4], col: usize) -> [u8; 4] {
        let mut ret = [0; 4];
        for i in 0..4 {
            ret[i] = (
                  FF::new(arr[(i+0)%4][col]) * FF::new(0x0e)
                + FF::new(arr[(i+1)%4][col]) * FF::new(0x0b)
                + FF::new(arr[(i+2)%4][col]) * FF::new(0x0d)
                + FF::new(arr[(i+3)%4][col]) * FF::new(0x09)
            ).value();
        }

        ret
    }

	pub fn add_round_key(&self, slice: &[u32]) -> State {

		let mut ret = [[0;4]; 4];
		for c in 0..4 {
			let word = util::bytes_to_word((self.state[0][c],self.state[1][c],self.state[2][c],self.state[3][c]));

			let res = word ^ slice[c];
			let bytes = util::word_to_bytes(res);

			ret[0][c] = bytes.0;
			ret[1][c] = bytes.1;
			ret[2][c] = bytes.2;
			ret[3][c] = bytes.3;
		}

		State{state: ret}
	}

	pub fn to_byte_array(self) -> [u8; 16] {
		let mut ret = [0; 16];

		for c in 0..4 {
			for r in 0..4 {
				ret[c*4 + r] = self.state[r][c];
			}
		}

		ret
}

	fn to_u128(&self) -> u128 {
		let mut res = 0;

		for c in 0..4 {
			for r in 0..4 {
				res <<= 8;
				res |= self.state[r][c] as u128;
			}
		}

		res
	}
}

impl fmt::Display for State {
	fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
		write!(formatter, "{:0>32x}", self.to_u128())
	}
}

#[cfg(test)]
mod tests {
    use state::*;

    static TEST_STATE: State = State{ state: [
        [0x19,0xa0,0x9a,0xe9],
        [0x3d,0xf4,0xc6,0xf8],
        [0xe3,0xe2,0x8d,0x48],
        [0xbe,0x2b,0x2a,0x08]
    ]};

    // The test cases I've been given for shift_rows, mix_columns
    // and add_round_key are each based on the output of the previous
    // function, so the later tests are dependent on the previous
    // functions being correctly implemented.  It's not exactly a
    // "unit test" per-se, but it's fine
    #[test]
    fn test_sub_bytes() {
        assert_eq!(TEST_STATE.sub_bytes(), State{ state: [
            [0xd4,0xe0,0xb8,0x1e],
            [0x27,0xbf,0xb4,0x41],
            [0x11,0x98,0x5d,0x52],
            [0xae,0xf1,0xe5,0x30]
        ]});
    }

    #[test]
    fn test_shift_rows() {
        assert_eq!(TEST_STATE.sub_bytes().shift_rows(), State{ state: [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0xbf, 0xb4, 0x41, 0x27],
            [0x5d, 0x52, 0x11, 0x98],
            [0x30, 0xae, 0xf1, 0xe5]
        ]});
		assert_eq!(State::from_slice(&[
			0x63,0xca,0xb7,0x04,
			0x09,0x53,0xd0,0x51,
			0xcd,0x60,0xe0,0xe7,
			0xba,0x70,0xe1,0x8c
		]).shift_rows(), State::from_slice(&[
			0x63,0x53,0xe0,0x8c,
			0x09,0x60,0xe1,0x04,
			0xcd,0x70,0xb7,0x51,
			0xba,0xca,0xd0,0xe7
		]));
    }

    #[test]
    fn test_inv_shift_rows() {
		assert_eq!(State::from_slice(&[
			0x63,0x53,0xe0,0x8c,
			0x09,0x60,0xe1,0x04,
			0xcd,0x70,0xb7,0x51,
			0xba,0xca,0xd0,0xe7
		]).inv_shift_rows(), State::from_slice(&[
			0x63,0xca,0xb7,0x04,
			0x09,0x53,0xd0,0x51,
			0xcd,0x60,0xe0,0xe7,
			0xba,0x70,0xe1,0x8c
		]));
    }

    #[test]
    fn test_shift_row() {
        assert_eq!([1, 2, 3, 4], State::shift_row(&([1,2,3,4] as [u8; 4]), 0));
		assert_eq!([2, 3, 4, 1], State::shift_row(&([1,2,3,4] as [u8; 4]), 1));
        assert_eq!([3, 4, 1, 2], State::shift_row(&([1,2,3,4] as [u8; 4]), 2));
		assert_eq!([4, 1, 2, 3], State::shift_row(&([1,2,3,4] as [u8; 4]), 3));
    }

    #[test]
    fn test_inv_shift_row() {
        assert_eq!([1,2,3,4], State::inv_shift_row(&([1, 2, 3, 4] as [u8; 4]), 0));
		assert_eq!([1,2,3,4], State::inv_shift_row(&([2, 3, 4, 1] as [u8; 4]), 1));
        assert_eq!([1,2,3,4], State::inv_shift_row(&([3, 4, 1, 2] as [u8; 4]), 2));
		assert_eq!([1,2,3,4], State::inv_shift_row(&([4, 1, 2, 3] as [u8; 4]), 3));
    }

    #[test]
    fn test_mix_columns() {
        assert_eq!(TEST_STATE.sub_bytes().shift_rows().mix_columns(), State{ state: [
            [0x04, 0xe0, 0x48, 0x28],
            [0x66, 0xcb, 0xf8, 0x06],
            [0x81, 0x19, 0xd3, 0x26],
            [0xe5, 0x9a, 0x7a, 0x4c]
        ]});
    }

    #[test]
    fn test_inv_mix_columns() {
        assert_eq!(State::from_slice(&[
            0x62, 0x7b, 0xce, 0xb9,
            0x99, 0x9d, 0x5a, 0xaa,
            0xc9, 0x45, 0xec, 0xf4,
            0x23, 0xf5, 0x6d, 0xa5
        ]).inv_mix_columns(), State::from_slice(&[
            0xe5, 0x1c, 0x95, 0x02,
            0xa5, 0xc1, 0x95, 0x05,
            0x06, 0xa6, 0x10, 0x24,
            0x59, 0x6b, 0x2b, 0x07
        ]));
    }

	#[test]
	fn test_add_round_key() {
		assert_eq!(State{state: [
			[0x04, 0xe0, 0x48, 0x28],
			[0x66, 0xcb, 0xf8, 0x06],
			[0x81, 0x19, 0xd3, 0x26],
			[0xe5, 0x9a, 0x7a, 0x4c]
		]}.add_round_key(&[0xa0fafe17, 0x88542cb1, 0x23a33939, 0x2a6c7605]), State{ state: [
			[0xa4, 0x68, 0x6b, 0x02],
			[0x9c, 0x9f, 0x5b, 0x6a],
			[0x7f, 0x35, 0xea, 0x50],
			[0xf2, 0x2b, 0x43, 0x49]
		]});
		assert_eq!(State::from_slice(&[
			0x00,0x11,0x22,0x33,
			0x44,0x55,0x66,0x77,
			0x88,0x99,0xaa,0xbb,
			0xcc,0xdd,0xee,0xff
		]).add_round_key(&[
			0x00010203,0x04050607,0x08090a0b,0x0c0d0e0f
		]), State::from_slice(&[
			0x00,0x10,0x20,0x30,
			0x40,0x50,0x60,0x70,
			0x80,0x90,0xa0,0xb0,
			0xc0,0xd0,0xe0,0xf0
		]));
	}

	#[test]
	fn test_to_u128() {
		assert_eq!(0x193de3bea0f4e22b9ac68d2ae9f84808, TEST_STATE.to_u128());
	}
}