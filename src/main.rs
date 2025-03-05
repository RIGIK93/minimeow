mod engine;

use std::{collections::VecDeque, fs::File, io, str::FromStr};
use chess::{Game, Board, ChessMove, MoveGen};
use engine::engine::Engine;
use engine::minimax::maxi;
use engine::move_tree::MoveTree;
use engine::uci::{uci_go, uci_set_position};
use engine::*;
use engine::evaluation::*;
use ptree::write_tree;
use engine::print_board::print_board;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut game = Game::new();
    let mut engine = Engine::new(3);

    while buffer != "quit\n" {
        buffer = String::new();
        stdin.read_line(&mut buffer)?;
        let mut tokens: VecDeque<&str> = buffer.trim().split_whitespace().collect();
        while tokens.len() != 0 {
            match tokens.pop_front().unwrap() {
                "uci" => {println!("id name MiniMeow\nid author RIGIK93\nuciok")},
                "isready" => {println!("readyok")},
                "quit" => return Ok(()),
                "ucinewgame" => {game = Game::new()},
                "position" => {uci_set_position(&mut game, &mut tokens);},
                "go" => {uci_go(&mut game, &mut engine, &mut tokens);},
                "stop" => {println!("bestmove {}", engine.stop().to_string());}
                "print" => {print_board(&game.current_position());},
                _ => {}
            }
        }
    }

    Ok(())
}

#[test]
#[should_panic]
fn panic_test() {
    panic!("I be panicking!");
}

#[test]
fn tree_gen_test() {
    let board = Board::default();
    let mv = MoveGen::new_legal(&board).last().unwrap();
    let mut tree = MoveTree::new(mv, default_eval, &board);
    let children = tree.gen_children();
    print_board(&tree.board);
    for c in &children {
        print_board(&c.board);
    }
    println!("{}", children.len());
    assert_eq!(children.len(), 20);
}

#[test]
fn basic_queen_taking_test() {
    let board: Board = Board::from_str("1B2k3/8/3q4/5Q2/8/8/4K3/8 w - - 0 1").unwrap();
    print_board(&board);

    let mut best = SMALL_EVAL;
    let mut best_mv = ChessMove::default();

    // f5b5
    // b8d6
    let moves = vec![ChessMove::from_str("f5b5").unwrap(), ChessMove::from_str("b8d6").unwrap()];
    for mv in moves {
        let tree = MoveTree::new(mv, default_eval, &board);
        let tmp = maxi(&tree, 3);

        println!("{}: {}", mv.to_string(), tmp);
        // write_tree(&tree, File::create(&mv.to_string()).unwrap()).unwrap();

        if tmp > best {
            best = tmp;
            best_mv = mv;
        }
    }
    // game.make_move(best_mv);
    assert_eq!(best_mv.to_string(), "b8d6");
    println!("----------");
    print_board(&board.make_move_new(best_mv));
}

#[test]
fn engine_test() {
    let mut eng = Engine::new(3);

    eng.start(Board::default());

    let mv = eng.stop();

}
