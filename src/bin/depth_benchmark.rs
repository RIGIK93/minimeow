use std::time::Instant;

use chess::{Board, ChessMove};
use chess_rust::engine::engine::Engine;


fn main() {
    let pos = Board::default();
    let pos = pos.make_move_new(ChessMove::from_san(&pos, "e4").unwrap());
    let pos = pos.make_move_new(ChessMove::from_san(&pos, "e5").unwrap());

    let mut depth = 2;

    let mut start_time = Instant::now();

    while start_time.elapsed().as_secs() <= 15 {
        let mut engine = Engine::new(depth);

        start_time = Instant::now();

        engine.test_calculate(&pos);

        println!("Searched depth {}, took {} seconds", depth, start_time.elapsed().as_secs_f32());

        depth += 1;
    }

    println!("Max depth until it takes more than 15 seconds to search is {}", depth - 2);
}