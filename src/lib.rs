pub mod bitboard {

    pub const EMPTY_STATE: u64 = 0;
    pub static EMPTY_BITBOARD: BitBoard = BitBoard {state: EMPTY_STATE};

    #[derive(Default, Debug)]
    pub struct BitBoard {
        state: u64,
    }
    impl BitBoard {
        pub fn new() -> BitBoard {
            BitBoard {state: EMPTY_STATE}
        }
    }

    impl PartialEq for BitBoard {
        fn eq(&self, other: &BitBoard) -> bool {
            self.state == other.state
        }
    }

    #[test]
    fn by_default_a_new_bitboard_is_empty() {
        let bb = BitBoard::new();
        assert_eq!(bb.state, EMPTY_STATE);
        assert_eq!(bb == EMPTY_BITBOARD, true);
    }
}
