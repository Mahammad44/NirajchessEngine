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

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Colour {
    White = 0,
    Black = 1,
}

#[derive(Clone)]
pub struct Board {
    // [colour] [piece]
    pub pieces: [[u64; 6]; 2],
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: [[0u64; 6]; 2],
        }
    }

    pub fn reset(&mut self) {
        *self = Board::new();
    }
}

fn print_bitboard(bitboard: u64) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let char = if bitboard & mask != 0 { '1' } else { '0' };
            print!("{char}");
        }
    }
    println!()
}
