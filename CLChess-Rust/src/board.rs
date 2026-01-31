/*
 *
 * board.rs module
 *
 * Contains board functionality
 *
*/

const COL_A : u64 = 0x0101010101010101;
const COL_B : u64 = COL_A << 1;
const COL_C : u64 = COL_B << 1;
const COL_D : u64 = COL_C << 1;
const COL_E : u64 = COL_D << 1;
const COL_F : u64 = COL_E << 1;
const COL_G : u64 = COL_F << 1;
const COL_H : u64 = COL_G << 1;

const ROW_1 : u64 = 0xFF;
const ROW_2 : u64 = ROW_1<<8;
const ROW_3 : u64 = ROW_2<<8;
const ROW_4 : u64 = ROW_3<<8;
const ROW_5 : u64 = ROW_4<<8;
const ROW_6 : u64 = ROW_5<<8;
const ROW_7 : u64 = ROW_6<<8;
const ROW_8 : u64 = ROW_7<<8;

pub struct Board {
        white_pawns     : u64,
        white_knights   : u64,
        white_bishops   : u64,
        white_rooks     : u64,
        white_queens    : u64,
        white_king      : u64,
       
        black_pawns     : u64,
        black_knights   : u64,
        black_bishops   : u64,
        black_rooks     : u64,
        black_queens    : u64,
        black_king      : u64
}

impl Board{
    fn new() -> Self {
        Self {
            white_pawns     : ROW_2,
            white_knights   : ROW_1 & (COL_B | COL_G),
            white_bishops   : ROW_1 & (COL_C | COL_F),
            white_rooks     : ROW_1 & (COL_A | COL_H),
            white_queens    : ROW_1 & COL_D,
            white_king      : ROW_1 & COL_E,
           
            black_pawns     : ROW_7,
            black_knights   : ROW_8 & (COL_B | COL_G),
            black_bishops   : ROW_8 & (COL_C | COL_F),
            black_rooks     : ROW_8 & (COL_A | COL_H),
            black_queens    : ROW_8 & COL_D,
            black_king      : ROW_8 & COL_E
        }
    }

    fn printBoard(&self) {
        let mut temp : u8 = 0;
        for ii in 0..8 {
            u8 = 
        }
    }
}
