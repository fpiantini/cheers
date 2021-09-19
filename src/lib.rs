pub mod bitboard {

    pub const EMPTY_STATE: u64 = 0;
    pub static EMPTY_BITBOARD: BitBoard = BitBoard {state: EMPTY_STATE};

    enum File
    {
        FileA, FileB, FileC, FileD, FileE, FileF, FileG, FileH,
    }
    enum Rank
    {
        Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8,
    }

    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct BitBoard {
        state: u64,
    }
    impl BitBoard {
        pub fn new() -> BitBoard {
            BitBoard {state: EMPTY_STATE}
        }
    }


    // TESTS ---------------------------------------------------------
    // Tests for File and Rank are quite obvious and can be removed in the future...
    #[test]
    fn file_a_has_index_0() {
        assert_eq!(File::FileA as i32, 0);
    }
    #[test]
    fn file_b_has_index_1() {
        assert_eq!(File::FileB as i32, 1);
    }
    #[test]
    fn file_c_has_index_2() {
        assert_eq!(File::FileC as i32, 2);
    }
    #[test]
    fn file_d_has_index_3() {
        assert_eq!(File::FileD as i32, 3);
    }
    #[test]
    fn file_e_has_index_4() {
        assert_eq!(File::FileE as i32, 4);
    }
    #[test]
    fn file_f_has_index_5() {
        assert_eq!(File::FileF as i32, 5);
    }
    #[test]
    fn file_g_has_index_6() {
        assert_eq!(File::FileG as i32, 6);
    }
    #[test]
    fn file_h_has_index_7() {
        assert_eq!(File::FileH as i32, 7);
    }

    #[test]
    fn rank_1_has_index_0() {
        assert_eq!(Rank::Rank1 as i32, 0);
    }
    #[test]
    fn rank_2_has_index_1() {
        assert_eq!(Rank::Rank2 as i32, 1);
    }
    #[test]
    fn rank_3_has_index_2() {
        assert_eq!(Rank::Rank3 as i32, 2);
    }
    #[test]
    fn rank_4_has_index_3() {
        assert_eq!(Rank::Rank4 as i32, 3);
    }
    #[test]
    fn rank_5_has_index_4() {
        assert_eq!(Rank::Rank5 as i32, 4);
    }
    #[test]
    fn rank_6_has_index_5() {
        assert_eq!(Rank::Rank6 as i32, 5);
    }
    #[test]
    fn rank_7_has_index_6() {
        assert_eq!(Rank::Rank7 as i32, 6);
    }
    #[test]
    fn rank_8_has_index_7() {
        assert_eq!(Rank::Rank8 as i32, 7);
    }

    #[test]
    fn by_default_a_new_bitboard_is_empty() {
        let bb = BitBoard::new();
        assert_eq!(bb.state, EMPTY_STATE);
        assert_eq!(bb, EMPTY_BITBOARD);
    }
    // TESTS ---------------------------------------------------------
}
