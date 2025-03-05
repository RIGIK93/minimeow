use chess::{Board, Piece, Square};


pub fn print_board(board: &Board) {
    let mut custom: String = String::new();

    for i in 0..64 {
        let sq = unsafe { Square::new(i) };
        let piece = board.piece_on(sq);

        let file: &str = match i % 8 {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => unreachable!(),
        };

        let rank = (i / 8 + 1) as u32;

        if file == "a" {
            custom.push_str(format!("{} ", rank).as_str());
        }

        if let Some(p) = piece {
            let mut piece = String::from(match p {
                Piece::Bishop => "B ",
                Piece::King => "K ",
                Piece::Knight => "N ",
                Piece::Pawn => "P ",
                Piece::Queen => "Q ",
                Piece::Rook => "R ",
            });

            piece = match board.color_on(sq).unwrap() {
                chess::Color::White => piece.to_uppercase(),
                chess::Color::Black => piece.to_lowercase(),
            };

            // custom.push_str(format!("{}{}.{},{} ", file, rank, color, piece).as_str());
            custom.push_str(&piece);
        } else {
            if rank % 2 == 0 {
                if (i % 8) % 2 == 0 {
                    custom.push_str("# ");
                } else {
                    custom.push_str("  ");
                }
            } else {
                if (i % 8) % 2 == 1 {
                    custom.push_str("# ");
                } else {
                    custom.push_str("  ");
                }
            }
        }

        // Add \ at the end of the file
        if file == "h" {
            custom.push_str("\n");
        }
    }

    custom.push_str("  a b c d e f g h");

    // let en = ChessTurnEngine::new(Setup::Custom(&*fc)).unwrap();
    // // println!("{}", en.display(DisplayOption::BoardView(ViewMode::FancyTui)));
    // en.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));
    println!("{}", custom);
}
