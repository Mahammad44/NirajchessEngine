use std::sync::{Arc, atomic::AtomicBool};
use crate::chess::core::board::Board;
use crate::chess::core::chessMove::ChessMove;

pub fn search(board: &Board, stop: Arc<AtomicBool>) -> ChessMove {
    unimplemented!()
}
