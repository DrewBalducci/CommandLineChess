/*
 * 
 * main.rs
 *
 * Main module 
 *
*/


mod board;





fn main() {
    let mut chessboard : board::Board = board::Board::new();
    println!("White: \u{2654}\nBlack: \u{265A}");
    chessboard.print_board();
}
