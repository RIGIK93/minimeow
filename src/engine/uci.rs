use std::{collections::VecDeque, str::FromStr};

use chess::{Board, ChessMove, Game};

use super::engine::Engine;


pub fn uci_set_position(game: &mut Game, args: &mut VecDeque<&str>) {

    let mut fen = String::new();
    let mut moves: Vec<ChessMove> = Vec::new();
    let mut moves_started = false; // becomes true when "args" reach "move" word

    loop {
        if let Some(fen_piece) = args.pop_front() {
            if fen_piece == "moves" {
                moves_started = true;
                break;
            }

            fen.push_str(fen_piece);
        } else {
            break;
        }
    }

    while let Some(mv) = args.pop_front() {
        moves.push(ChessMove::from_str(mv).unwrap());
    }

    if fen == "startpos" {
        *game = Game::new_with_board(Board::default());
    } else {
        *game = Game::new_with_board(Board::from_str(&fen).unwrap());
    }

    for mv in moves {
        game.make_move(mv);
    }
}

pub fn uci_go(game: &mut Game, engine: &mut Engine, args: &mut VecDeque<&str>) -> Option<ChessMove> {
    if args.pop_front().unwrap_or_default() == "infinite" {
        engine.start(game.current_position());
    }

    None
}
