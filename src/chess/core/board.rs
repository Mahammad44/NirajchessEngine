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
#[derive(Copy, Clone)]
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
    // [colour] [piece]
    pub bb_pieces: [[u64; 6]; 2],
    pub bb_side: [Bitboard; Sides::BOTH], // holds two bitboards
}

impl Board {
    pub fn new() -> Self {
        Self {
            bb_pieces: [[0; 6]; 2],
            bb_side: [0; 2],
        }
    }

    pub fn reset(&mut self) {
        *self = Board::new();
    }

    pub fn get_pieces(&self, side: Side, piece: Piece) -> Bitboard {
        self.bb_pieces[side][piece]
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
        board.bb_pieces[Sides::WHITE][Pieces::Pawn as usize] = 1u64 << 12;

        print_bitboard(board.get_pieces(
            Sides::WHITE,
            Pieces::Pawn as usize,
        ));
    }
}


