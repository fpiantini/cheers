pub mod bitboard {
    pub struct BitBoard {
        pub state: u64,
    }

    #[test]
    fn declare_empty_bitboard() {
        let bb = BitBoard {state: 0};
        assert_eq!(bb.state, 0);
    }
}
