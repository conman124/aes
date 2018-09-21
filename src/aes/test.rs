mod tests {
    use aes::*;

    static TEST_STATE: State = State{ state: [
        [0x19,0xa0,0x9a,0xe9],
        [0x3d,0xf4,0xc6,0xf8],
        [0xe3,0xe2,0x8d,0x48],
        [0xbe,0x2b,0x2a,0x08]
    ]};

    #[test]
    fn test_word_to_bytes() {
        assert_eq!((0xde, 0xad, 0xbe, 0xef), word_to_bytes(0xdeadbeef));
    }

    #[test]
    fn test_bytes_to_word() {
        assert_eq!(0xdeadbeef, bytes_to_word((0xde, 0xad, 0xbe, 0xef)));
    }

    #[test]
    fn test_sub_word() {
        assert_eq!(sub_word(0x00102030), 0x63cab704);
        assert_eq!(sub_word(0x40506070), 0x0953d051);
        assert_eq!(sub_word(0x8090a0b0), 0xcd60e0e7);
        assert_eq!(sub_word(0xc0d0e0f0), 0xba70e18c);
    }

    #[test]
    fn test_sub_byte() {
        assert_eq!(sub_byte(0x40), 0x09);
        assert_eq!(sub_byte(0x50), 0x53);
        assert_eq!(sub_byte(0x60), 0xd0);
        assert_eq!(sub_byte(0x70), 0x51);
    }

    #[test]
    fn test_rot_word() {
        assert_eq!(rot_word(0x09cf4f3c), 0xcf4f3c09);
        assert_eq!(rot_word(0x2a6c7605), 0x6c76052a);
    }

    #[test]
    fn test_key_schedule() {
        let input = vec![0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c];
        let key = Key::new(input.clone());
        let schedule = key.create_schedule();

        let expected = vec![
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

        assert_eq!(KeySchedule::new(expected), schedule);
    }

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
    }

    #[test]
    fn test_shift_row() {
        assert_eq!([1, 2, 3, 4], State::shift_row(&([1,2,3,4] as [u8; 4]), 0));
        assert_eq!([3, 4, 1, 2], State::shift_row(&([1,2,3,4] as [u8; 4]), 2));
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
}