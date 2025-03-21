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

// pub fn mva_lvv() {

// }
