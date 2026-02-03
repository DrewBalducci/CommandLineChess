/*
 *
 * board.rs module
 *
 * Contains board functionality
 *
*/

#[allow(dead_code)]

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

#[derive(PartialEq)]
pub enum Colors {
    WhitePlayer,
    BlackPlayer,
    LightMode,
    DarkMode
}

#[derive(Clone)]
enum PieceChars {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
   
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing
}

impl PieceChars {
    fn get_char(self, color_mode : &Colors) -> char {
        let mut char_data : u32;
        let mut return_val : char;
        
        match self {
            PieceChars::WhitePawn => char_data = u32::from('\u{2659}'),
            PieceChars::WhiteKnight => char_data = u32::from('\u{2658}'),
            PieceChars::WhiteBishop => char_data = u32::from('\u{2657}'),
            PieceChars::WhiteRook => char_data = u32::from('\u{2656}'),
            PieceChars::WhiteQueen => char_data = u32::from('\u{2655}'),
            PieceChars::WhiteKing => char_data = u32::from('\u{2654}'),
           
            PieceChars::BlackPawn => char_data = u32::from('\u{265F}'),
            PieceChars::BlackKnight => char_data = u32::from('\u{265E}'),
            PieceChars::BlackBishop => char_data = u32::from('\u{265D}'),
            PieceChars::BlackRook => char_data = u32::from('\u{265C}'),
            PieceChars::BlackQueen => char_data = u32::from('\u{265B}'),
            PieceChars::BlackKing => char_data = u32::from('\u{265A}'),
        }
        if (*color_mode == Colors::DarkMode) {
            if (char_data > u32::from('\u{2659}')) {
                char_data = char_data - 6;
            } else {
                char_data = char_data + 6;
            }
        }
        return_val = std::char::from_u32(char_data).expect("REASON");
        return return_val;
    }
}

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
    black_king      : u64,

    term_theme : Colors,
    player_color : Colors
}

impl Board{
    pub fn new() -> Self {
        Self {
            white_pawns     : ROW_7,
            white_knights   : ROW_8 & (COL_B | COL_G),
            white_bishops   : ROW_8 & (COL_C | COL_F),
            white_rooks     : ROW_8 & (COL_A | COL_H),
            white_queens    : ROW_8 & COL_D,
            white_king      : ROW_8 & COL_E,
           
            black_pawns     : ROW_2,
            black_knights   : ROW_1 & (COL_B | COL_G),
            black_bishops   : ROW_1 & (COL_C | COL_F),
            black_rooks     : ROW_1 & (COL_A | COL_H),
            black_queens    : ROW_1 & COL_D,
            black_king      : ROW_1 & COL_E,

            term_theme      : Colors::DarkMode,
            player_color    : Colors::WhitePlayer
        }
    }

    
    pub fn set_theme(&mut self, theme : Colors) {
        self.term_theme = theme;
    }

    pub fn toggle_theme(&mut self) {
        if (self.term_theme == Colors::DarkMode) {
            self.term_theme = Colors::LightMode;
        } else {
            self.term_theme = Colors::DarkMode;
        }
    }
    

    pub fn set_color(&mut self, player_color : Colors) {
        self.player_color = player_color;
    }

    pub fn toggle_color(&mut self) {
        if (self.player_color == Colors::WhitePlayer) {
            self.player_color = Colors::BlackPlayer;
        } else {
            self.player_color = Colors::WhitePlayer;
        }
    }



    pub fn print_bitboard(bit_board : u64){
        println!("   A B C D E F G H");
        let mut temp : u64;
        for mut row in 0..8 {
            print!(" {} ", row+1);
            for col in 0..8 {
                temp = (1 << (8*row)) << col;
                print!("{} ", temp);
            }
            println!("");
        }

    }

    pub fn print_board(&self) {
        println!("   A B C D E F G H");
        let mut mask : u64;
        for mut row in 0..8 {
            if (self.player_color == Colors::BlackPlayer) {row = row - 1;}
            print!(" {} ", row+1);
            for col in 0..8 {
                mask = (1 << (8*row)) << col;
                if (self.white_pawns & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::WhitePawn, &self.term_theme));}
                else if (self.white_knights & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::WhiteKnight, &self.term_theme));}
                else if (self.white_bishops & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::WhiteBishop, &self.term_theme));}
                else if (self.white_rooks & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::WhiteRook, &self.term_theme));}
                else if (self.white_queens & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::WhiteQueen, &self.term_theme));}
                else if (self.white_king & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::WhiteKing, &self.term_theme));}
                else if (self.black_pawns & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::BlackPawn, &self.term_theme));}
                else if (self.black_knights & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::BlackKnight, &self.term_theme));}
                else if (self.black_bishops & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::BlackBishop, &self.term_theme));}
                else if (self.black_rooks & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::BlackRook, &self.term_theme));}
                else if (self.black_queens & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::BlackQueen, &self.term_theme));}
                else if (self.black_king & mask != 0) {print!("{} ",PieceChars::get_char(PieceChars::BlackKing, &self.term_theme));}
                else {print!("- ");}
            }
            println!("");
        }

    }
}
