mod engine;

use std::{collections::VecDeque, io};
use chess::Game;
use engine::engine::Engine;
use engine::print_board::print_board;
use engine::uci::{uci_go, uci_set_position};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut game = Game::new();
    let mut engine = Engine::new(7);

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
