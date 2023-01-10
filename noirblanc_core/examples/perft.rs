use std::env::args;

use noirblanc_core::{perft, Position};

fn main() {
    let args: Vec<_> = args().collect();
    let mut depth = 5;
    if args.len() >= 2 {
        depth = args[1].parse().unwrap();
    }
    let pos = Position::startpos();
    let result = perft::perft(pos, depth, false);
    println!("{} => {}", depth, result.all);
}
