use chess::{Board, Color, Piece, Square};

pub type EvalFunc = fn(&Board) -> CP;

/// Centipawns
pub type CP = i16;

pub const LARGE_EVAL: CP = 10000;
pub const SMALL_EVAL: CP = -LARGE_EVAL;

/// The default eval just counts material
/// TODO: The function will be renamed into material_count, due to its 
/// replacement with a more sophisticated evaluation function
pub fn material_eval(b: &Board) -> CP {
    let mut sq = Square::A1;
    let mut eval: CP = 0;
    
    loop {
        if let Some(p) = b.piece_on(sq) {
            let tmp: CP = match p {
                Piece::Bishop => 300,
                Piece::Knight => 300,
                Piece::King => LARGE_EVAL,
                Piece::Pawn => 100,
                Piece::Queen => 900,
                Piece::Rook => 500,
            };

            match b.color_on(sq).unwrap() {
                Color::White => eval += tmp,
                Color::Black => eval -= tmp,
            }
        }

        // scan the board right until the last file, then up
        match sq.right() {
            Some(s) => sq = s,
            None => match sq.up() {
                Some(u) => sq = u.uright(),
                None => break,
            },
        }
    }

    eval
}

const PAWN_TABLE: [[i16; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [ 5,  5, 10, 25, 25, 10,  5,  5],
    [ 0,  0,  0, 20, 20,  0,  0,  0],
    [ 5, -5,-10,  0,  0,-10, -5,  5],
    [ 5, 10, 10,-20,-20, 10, 10,  5],
    [ 0,  0,  0,  0,  0,  0,  0,  0]
];

const KNIGHT_TABLE: [[i16; 8]; 8] = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20,   0,   0,   0,   0, -20, -40],
    [-30,   0,  10,  15,  15,  10,   0, -30],
    [-30,   5,  15,  20,  20,  15,   5, -30],
    [-30,   0,  15,  20,  20,  15,   0, -30],
    [-30,   5,  10,  15,  15,  10,   5, -30],
    [-40, -20,   0,   5,   5,   0, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50]
];

const BISHOP_TABLE: [[i16; 8]; 8] = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10,   0,   0,   0,   0,   0,   0, -10],
    [-10,   0,   5,  10,  10,   5,   0, -10],
    [-10,   5,   5,  10,  10,   5,   5, -10],
    [-10,   0,  10,  10,  10,  10,   0, -10],
    [-10,  10,  10,  10,  10,  10,  10, -10],
    [-10,   5,   0,   0,   0,   0,   5, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20]
];

const ROOK_TABLE: [[i16; 8]; 8] = [
    [ 0,  0,  0,  0,  0,  0,  0,  0],
    [ 5, 10, 10, 10, 10, 10, 10,  5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [ 0,  0,  0,  5,  5,  0,  0,  0]
];

const QUEEN_TABLE: [[i16; 8]; 8] = [
    [-20, -10, -10,  -5,  -5, -10, -10, -20],
    [-10,   0,   0,   0,   0,   0,   0, -10],
    [-10,   0,   5,   5,   5,   5,   0, -10],
    [ -5,   0,   5,   5,   5,   5,   0,  -5],
    [  0,   0,   5,   5,   5,   5,   0,  -5],
    [-10,   5,   5,   5,   5,   5,   0, -10],
    [-10,   0,   5,   0,   0,   0,   0, -10],
    [-20, -10, -10,  -5,  -5, -10, -10, -20]
];

const KING_MIDGAME_TABLE: [[i16; 8]; 8] = [
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-20, -30, -30, -40, -40, -30, -30, -20],
    [-10, -20, -20, -20, -20, -20, -20, -10],
    [ 20,  20,   0,   0,   0,   0,  20,  20],
    [ 20,  30,  10,   0,   0,  10,  30,  20]
];

const KING_ENDGAME_TABLE: [[i16; 8]; 8] = [
    [-50, -40, -30, -20, -20, -30, -40, -50],
    [-30, -20, -10,   0,   0, -10, -20, -30],
    [-30, -10,  20,  30,  30,  20, -10, -30],
    [-30, -10,  30,  40,  40,  30, -10, -30],
    [-30, -10,  30,  40,  40,  30, -10, -30],
    [-30, -10,  20,  30,  30,  20, -10, -30],
    [-30, -30,   0,   0,   0,   0, -30, -30],
    [-50, -30, -30, -30, -30, -30, -30, -50]
];


pub fn piece_square_material_eval(b: &Board) -> CP {
    let mut sq = Square::A1;
    let mut eval: CP = 0;
    
    loop {
        if let Some(p) = b.piece_on(sq) {
            let color = b.color_on(sq).unwrap();

            let y = match color {
                Color::White => 7 - sq.get_rank().to_index(),
                Color::Black => sq.get_rank().to_index()
            };

            let x = match color {
                Color::White => sq.get_file().to_index(),
                Color::Black => 7 - sq.get_file().to_index()
            };

            let tmp: CP = match p {
                Piece::Bishop => 300 + BISHOP_TABLE[y][x],
                Piece::Knight => 300 + KNIGHT_TABLE[y][x],
                Piece::King => LARGE_EVAL + KING_MIDGAME_TABLE[y][x],
                Piece::Pawn => 100 + PAWN_TABLE[y][x],
                Piece::Queen => 900 + QUEEN_TABLE[y][x],
                Piece::Rook => 500 + ROOK_TABLE[y][x],
            };

            match color {
                Color::White => eval += tmp,
                Color::Black => eval -= tmp,
            }
        }

        // scan the board right until the last file, then up
        match sq.right() {
            Some(s) => sq = s,
            None => match sq.up() {
                Some(u) => sq = u.uright(),
                None => break,
            },
        }
    }

    eval
}

// making sure I understand the logic behind files and ranks
#[test]
fn square_to_number_logic() {
    let sq = Square::A1;

    assert_eq!(sq.get_file().to_index(), 0);
    assert_eq!(sq.get_rank().to_index(), 0)
}

#[test]
fn square_table_conversion() {
    let sq = Square::H1;

    assert_eq!(KING_MIDGAME_TABLE[7 - sq.get_rank().to_index()][sq.get_file().to_index()], 20);
}

// pub fn mva_lvv() {

// }
