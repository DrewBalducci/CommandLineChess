/*
 * 
 * main.rs
 *
 * Main module 
 *
*/


mod board;





fn main() {
    let mut chessboard : board::Board;
    for i in 0..8 {
        print!("");
    }
    println!("White: \u{2654}\nBlack: \u{265A}");
}
