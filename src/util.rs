pub fn word_to_bytes(word: u32) -> (u8, u8, u8, u8) {
    (
        ((word & 0xff000000) >> 24) as u8,
        ((word & 0x00ff0000) >> 16) as u8,
        ((word & 0x0000ff00) >> 08) as u8,
        ((word & 0x000000ff) >> 00) as u8
    )
}

pub fn bytes_to_word(bytes: (u8, u8, u8, u8)) -> u32 {
    (bytes.0 as u32) << 24 ^
    (bytes.1 as u32) << 16 ^
    (bytes.2 as u32) << 08 ^
    (bytes.3 as u32)
}

pub fn rot_word(word: u32) -> u32 {
    let high: u8 = ((word & 0xff000000) >> 24) as u8;
    let word = word << 8;

    word ^ (high as u32)
}

#[cfg(test)]
mod tests {
	use util::*;
	
	#[test]
    fn test_word_to_bytes() {
        assert_eq!((0xde, 0xad, 0xbe, 0xef), word_to_bytes(0xdeadbeef));
    }

    #[test]
    fn test_bytes_to_word() {
        assert_eq!(0xdeadbeef, bytes_to_word((0xde, 0xad, 0xbe, 0xef)));
    }

    #[test]
    fn test_rot_word() {
        assert_eq!(rot_word(0x09cf4f3c), 0xcf4f3c09);
        assert_eq!(rot_word(0x2a6c7605), 0x6c76052a);
    }
}