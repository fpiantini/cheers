pub mod bitboard {

    const EMPTY_STATE: u64 = 0;

    enum File
    {
        FileA, FileB, FileC, FileD, FileE, FileF, FileG, FileH,
    }
    enum Rank
    {
        Rank1, Rank2, Rank3, Rank4, Rank5, Rank6, Rank7, Rank8,
    }

    #[derive(Clone)]
    pub enum Cell
    {
        A1, B1, C1, D1, E1, F1, G1, H1,
        A2, B2, C2, D2, E2, F2, G2, H2,
        A3, B3, C3, D3, E3, F3, G3, H3,
        A4, B4, C4, D4, E4, F4, G4, H4,
        A5, B5, C5, D5, E5, F5, G5, H5,
        A6, B6, C6, D6, E6, F6, G6, H6,
        A7, B7, C7, D7, E7, F7, G7, H7,
        A8, B8, C8, D8, E8, F8, G8, H8,
    }

    #[derive(Default, Debug, PartialEq, Eq)]
    pub struct BitBoard {
        state: u64,
    }
    impl BitBoard {
        pub fn new() -> BitBoard {
            BitBoard {state: EMPTY_STATE}
        }
        pub fn is_empty(&self) -> bool {
            self.state == EMPTY_STATE
        }
        pub fn set_cell(& mut self, c: Cell) {
            self.state |= 1 << c as usize;
        }
    }

    // ------------------------------------------------------------
    // FIXME --- Seems impossible to add the From trait for a single
    // Cell because conflicts with the next From trait for Cell slices.
    // i.e. adding the following code cause compilation failure:
    //
    // impl From<Cell> for BitBoard {
    //     fn from(c: Cell) -> Self {
    //         let mut bb = BitBoard::new();
    //         bb.set_cell(c);
    //         bb
    //     }
    // }
    // Because of this, I was not able to add a method to create a
    // BitBoard using a single cell. To do this using the From slice
    // trait we have to do this (see tests):
    //    let bb = BitBoard::from([Cell::H8]);
    //
    // but these does not work:
    //    let bb = BitBoard::from(Cell::H8);
    //    let bb = BitBoard::from(&Cell::H8);
    // This implementation is the only way I found to work with
    // arrays, vectors and slices, but onestly I still do not understand
    // it. See:
    // https://www.reddit.com/r/rust/comments/70xqpw/using_the_from_trait_not_as_easy_as_i_thought/
    impl <'a, T: AsRef<[Cell]>> From<T> for BitBoard {
        fn from(cells: T) -> Self {
            let mut bb = BitBoard::new();
            for c in cells.as_ref().to_vec() {
                bb.set_cell(c);
            }
            bb
        }
    }
    // TESTS ---------------------------------------------------------
    #[test]
    fn by_default_a_new_bitboard_is_empty() {
        let bb = BitBoard::new();
        assert_eq!(bb.state, EMPTY_STATE);
        assert_eq!(bb.is_empty(), true);
    }
    #[test]
    fn init_bitboard_using_a_vector_with_a_cell_in_h8() {
        let bb = BitBoard::from([Cell::H8]);
        assert_eq!(bb.is_empty(), false);
        assert_eq!(bb.state, 0x80_00_00_00_00_00_00_00);
    }
    #[test]
    fn init_bitboard_using_a_cells_vector_with_active_cell_in_diagonal() {
        let bb = BitBoard::from([Cell::A1, Cell::B2, Cell::C3, Cell::D4,
             Cell::E5, Cell::F6, Cell::G7, Cell::H8, ]);
        assert_eq!(bb.is_empty(), false);
        assert_eq!(bb.state, 0x80_40_20_10_08_04_02_01);
    }
    // TESTS ---------------------------------------------------------
}
