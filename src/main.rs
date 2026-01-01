mod chess;
mod uci;

fn main() {
    uci::engineLoop::run_uci();
}
