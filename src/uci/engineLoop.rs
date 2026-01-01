use crate::chess::core::board::Board;
use crate::chess::search::search::search;
use crate::uci::parser::parse_position;
use std::fmt::Display;
use std::io::{self, BufRead, Write};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::thread;

pub fn run_uci() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let stop_flag = Arc::new(AtomicBool::new(false));

    let mut board = Board::new(); // board stuct

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace();

        match tokens.next() {
            Some("uci") => {
                writeln!(stdout, "id name Niraj").unwrap();
                writeln!(stdout, "id author Ibn Carlton").unwrap();
                // optional and i dont understand it or why yet: send options
                writeln!(stdout, "uciok").unwrap();
            }
            Some("ucinewgame") => {
                board.reset();
            }
            Some("position") => {
                // parse fen or startpos and moves
                parse_position(&mut board, tokens.collect::<Vec<_>>());
            }
            Some("go") => {
                stop_flag.store(false, Ordering::SeqCst);
                let board_clone = board.clone();

                let stop_clone = Arc::clone(&stop_flag);
                thread::spawn(move || {
                    let best = search(&board_clone, stop_clone);
                    println!("bestmove {:?}", best);
                });
            }
            Some("isready") => {
                writeln!(stdout, "readyok").unwrap();
            }
            Some("stop") => {
                stop_flag.store(true, Ordering::SeqCst);
            }
            Some("quit") => {
                break;
            }
            _ => {}
        }
    }
}
