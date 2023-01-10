use noirblanc_core::{Bitboard, Color, Position};

fn main() {
    let bit_board = Position(Bitboard(0x5555), Bitboard(0xaaaa), Color::Black);
    println!("{}", bit_board);
}
