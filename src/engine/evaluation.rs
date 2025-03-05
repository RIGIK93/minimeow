use chess::{Board, Color, Piece, Square};

pub type EvalFunc = fn(&Board) -> f64;

pub const LARGE_EVAL: f64 = 1000000000 as f64;
pub const SMALL_EVAL: f64 = -LARGE_EVAL;

/// The default eval just counts material
/// TODO: The function will be renamed into material_count, due to its 
/// replacement with a more sophisticated evaluation function
pub fn default_eval(b: &Board) -> f64 {
    let mut sq = Square::A1;
    let mut eval: f64 = 0.0;

    loop {
        if let Some(p) = b.piece_on(sq) {
            let tmp: f64 = match p {
                Piece::Bishop => 3.0,
                Piece::Knight => 3.0,
                Piece::King => LARGE_EVAL,
                Piece::Pawn => 1.0,
                Piece::Queen => 9.0,
                Piece::Rook => 5.0,
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
