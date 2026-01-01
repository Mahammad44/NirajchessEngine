use crate::chess::core::board::Board;
use crate::chess::core::chessMove::ChessMove;
use std::sync::{Arc, atomic::AtomicBool};

pub fn search(board: &Board, stop: Arc<AtomicBool>) -> ChessMove {
    unimplemented!()
}
