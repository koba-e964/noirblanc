use noirblanc_core::bit_board::BitBoard;

fn main() {
    let bit_board = BitBoard(0x5555, 0xaaaa, true);
    println!("{}", bit_board);
}
