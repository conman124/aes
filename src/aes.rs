use key::Key;
use state::State;

pub struct Encryptor {
	key: Key
}

pub struct Decryptor {
	key: Key
}

impl Encryptor {
	pub fn using(key: Key) -> Encryptor {
		Encryptor{key}
	}

	pub fn encrypt(&self, input: &[u8], debug: bool) -> [u8; 16] {
		if input.len() != 16 { panic!("Can only encrypt 16 byte blocks!"); }

		let key_schedule = self.key.create_schedule();

		let mut state = State::from_slice(input);
		if debug { println!("round[ 0].input    {}", state); }

		let ks0 = &key_schedule[0];
		if debug { println!("round[ 0].k_sch    {:0>8x}{:0>8x}{:0>8x}{:0>8x}", ks0[0], ks0[1], ks0[2], ks0[3])}
		state = state.add_round_key(ks0);

		let nr = match self.key.get_size_bits() {
			128 => {10},
			192 => {12},
			256 => {14},
			_ => {panic!("Invalid key size!")}
		};

		for round in 1..nr {
			if debug { println!("round[{: >2}].start    {}", round, state); }
			state = state.sub_bytes();
			if debug { println!("round[{: >2}].s_box    {}", round, state); }
			state = state.shift_rows();
			if debug { println!("round[{: >2}].s_row    {}", round, state); }
			state = state.mix_columns();
			if debug { println!("round[{: >2}].m_col    {}", round, state); }

			let ks = &key_schedule[round];
			if debug { println!("round[{: >2}].k_sch    {:0>8x}{:0>8x}{:0>8x}{:0>8x}", round, ks[0], ks[1], ks[2], ks[3])}
			state = state.add_round_key(ks);
		}

		if debug { println!("round[{}].start    {}", nr, state); }
		state = state.sub_bytes();
		if debug { println!("round[{}].s_box    {}", nr, state); }
		state = state.shift_rows();
		if debug { println!("round[{}].s_row    {}", nr, state); }

		let ks = &key_schedule[nr];
		if debug { println!("round[{}].k_sch    {:0>8x}{:0>8x}{:0>8x}{:0>8x}", nr, ks[0], ks[1], ks[2], ks[3]); }
		state = state.add_round_key(&key_schedule[nr]);
		if debug { println!("round[{}].output   {}", nr, state); }

		state.to_byte_array()
	}
}


impl Decryptor {
	pub fn using(key: Key) -> Decryptor {
		Decryptor{key}
	}

	pub fn decrypt(&self, input: &[u8], debug: bool) -> [u8; 16] {
		if input.len() != 16 { panic!("Can only decrypt 16 byte blocks!"); }

		let key_schedule = self.key.create_schedule();

		let mut state = State::from_slice(input);
		if debug { println!("round[ 0].iinput   {}", state); }

		let nr = match self.key.get_size_bits() {
			128 => {10},
			192 => {12},
			256 => {14},
			_ => {panic!("Invalid key size!")}
		};

		let ks = &key_schedule[nr];
		if debug { println!("round[ 0].ik_sch   {:0>8x}{:0>8x}{:0>8x}{:0>8x}", ks[0], ks[1], ks[2], ks[3])}
		state = state.add_round_key(ks);

		for round in (1..nr).rev() {
			if debug { println!("round[{: >2}].istart   {}", nr-round, state); }
			state = state.inv_shift_rows();
			if debug { println!("round[{: >2}].is_row   {}", nr-round, state); }
			state = state.inv_sub_bytes();
			if debug { println!("round[{: >2}].is_box   {}", nr-round, state); }

			let ks = &key_schedule[round];
			if debug { println!("round[{: >2}].ik_sch   {:0>8x}{:0>8x}{:0>8x}{:0>8x}", nr-round, ks[0], ks[1], ks[2], ks[3])}
			state = state.add_round_key(ks);
			if debug { println!("round[{: >2}].ik_add   {}", nr-round, state); }
			state = state.inv_mix_columns();
		}

		if debug { println!("round[{}].istart   {}", nr, state); }
		state = state.inv_shift_rows();
		if debug { println!("round[{}].is_row   {}", nr, state); }
		state = state.inv_sub_bytes();
		if debug { println!("round[{}].is_box   {}", nr, state); }

		let ks = &key_schedule[0];
		if debug { println!("round[{}].ik_sch   {:0>8x}{:0>8x}{:0>8x}{:0>8x}", nr, ks[0], ks[1], ks[2], ks[3]); }
		state = state.add_round_key(ks);
		if debug { println!("round[{}].ioutput  {}", nr, state); }

		state.to_byte_array()
	}
}

#[cfg(test)]
mod tests {
	use aes::*;

	#[test]
	fn test_encryptor128() {
		let key = Key::new(&[0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f]);
		let encryptor = Encryptor::using(key);

		let input = [
			0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
			0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
		];
		let expected = [
			0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30,
			0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5, 0x5a
		];
		assert_eq!(expected, encryptor.encrypt(&input, false));

		let key = Key::new(&[0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c]);
		let encryptor = Encryptor::using(key);

		let input = [
			0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d,
			0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07, 0x34
		];
		let expected = [
			0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb,
			0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b, 0x32
		];
		assert_eq!(expected, encryptor.encrypt(&input, false));
	}

	#[test]
	fn test_encryptor192() {
		let key= Key::new(&[0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f, 0x10111213, 0x14151617]);
		let encryptor = Encryptor::using(key);

		let input = [
			0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
			0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
		];
		let expected = [
			0xdd, 0xa9, 0x7c, 0xa4, 0x86, 0x4c, 0xdf, 0xe0,
			0x6e, 0xaf, 0x70, 0xa0, 0xec, 0x0d, 0x71, 0x91
		];
		assert_eq!(expected, encryptor.encrypt(&input, false));
	}

	#[test]
	fn test_encrypter256() {
		let key = Key::new(&[0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f, 0x10111213, 0x14151617, 0x18191a1b, 0x1c1d1e1f]);
		let encryptor = Encryptor::using(key);

		let input = [
			0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
			0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
		];
		let expected = [
			0x8e, 0xa2, 0xb7, 0xca, 0x51, 0x67, 0x45, 0xbf,
			0xea, 0xfc, 0x49, 0x90, 0x4b, 0x49, 0x60, 0x89
		];
		assert_eq!(expected, encryptor.encrypt(&input, false));
	}

	#[test]
	fn test_decryptor128() {
		let key = Key::new(&[0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f]);
		let decryptor = Decryptor::using(key);

		let input = [
			0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30,
			0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5, 0x5a
		];
		let expected = [
			0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
			0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
		];
		assert_eq!(expected, decryptor.decrypt(&input, true));
	}

	#[test]
	fn test_decryptor192() {
		let key = Key::new(&[0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f, 0x10111213, 0x14151617]);
		let decryptor = Decryptor::using(key);

		let input = [
			0xdd, 0xa9, 0x7c, 0xa4, 0x86, 0x4c, 0xdf, 0xe0,
			0x6e, 0xaf, 0x70, 0xa0, 0xec, 0x0d, 0x71, 0x91
		];
		let expected = [
			0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
			0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
		];
		assert_eq!(expected, decryptor.decrypt(&input, true));
	}

	#[test]
	fn test_decryptor256() {
		let key = Key::new(&[0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f, 0x10111213, 0x14151617, 0x18191a1b, 0x1c1d1e1f]);
		let decryptor = Decryptor::using(key);

		let input = [
			0x8e, 0xa2, 0xb7, 0xca, 0x51, 0x67, 0x45, 0xbf,
			0xea, 0xfc, 0x49, 0x90, 0x4b, 0x49, 0x60, 0x89
		];
		let expected = [
			0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
			0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
		];
		assert_eq!(expected, decryptor.decrypt(&input, true));
	}
}