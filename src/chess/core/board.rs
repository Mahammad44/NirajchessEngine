pub type Bitboard = u64;
pub type Side = usize;
pub type Piece = usize;

pub struct Sides;
impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const BOTH: usize = 2;
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Pieces {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    None = 6,
}

impl Pieces {
    pub fn as_usize(self) -> usize {
        self as usize
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Colour {
    White = 0,
    Black = 1,
}

#[derive(Clone)]
pub struct Board {
    // [colour][piece]
    pub bb_pieces: [[Bitboard; 6]; 2],
    pub bb_side: [Bitboard; Sides::BOTH],
    pub occupancy: [Bitboard; 2],
    pub piece_list: [Pieces; 64],
}

impl Board {
    pub fn new() -> Self {
        Self {
            bb_pieces: [[0; 6]; 2],
            bb_side: [0; 2],
            occupancy: [0; 2],
            piece_list: [Pieces::None; 64],
        }
    }

    pub fn reset(&mut self) {
        *self = Board::new();
    }

    pub fn get_pieces(&self, side: Side, piece: Piece) -> Bitboard {
        self.bb_pieces[side][piece]
    }

    /// Build the piece list from bitboards
    pub fn init_piece_list(&mut self) {
        self.piece_list = [Pieces::None; 64];

        for side in 0..2 {
            for piece in 0..6 {
                let mut bb = self.bb_pieces[side][piece];

                while bb != 0 {
                    let sq = bb.trailing_zeros() as usize;
                    bb &= bb - 1;

                    self.piece_list[sq] = match piece {
                        0 => Pieces::King,
                        1 => Pieces::Queen,
                        2 => Pieces::Rook,
                        3 => Pieces::Bishop,
                        4 => Pieces::Knight,
                        5 => Pieces::Pawn,
                        _ => Pieces::None,
                    };
                }
            }
        }
    }
}

pub fn print_bitboard(bitboard: u64) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let ch = if bitboard & mask != 0 { '1' } else { '0' };
            print!("{ch}");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn white_pawn_on_e2() {
        let mut board = Board::new();

        // E2 = square 12 in your mapping
        board.bb_pieces[Sides::WHITE][Pieces::Pawn as usize] = 1u64 << 12;

        board.init_piece_list();

        assert_eq!(board.piece_list[12], Pieces::Pawn);

        print_bitboard(board.get_pieces(
            Sides::WHITE,
            Pieces::Pawn as usize,
        ));
    }
}

