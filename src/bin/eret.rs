use std::{collections::VecDeque, error::Error, io, str::FromStr, time::Instant};

use chess::{Board, ChessMove};
use chess_rust::engine::engine::Engine;

#[derive(Debug)]
struct ChessPuzzle {
    board: Board,
    bm: ChessMove, /// bm stands for best move
    id: String,
}

fn puzzlify_epd() -> Result<Vec<ChessPuzzle>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    // let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut puzzles = Vec::new();

    for result in io::stdin().lines() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let s: String = result?;
        // let s: &str = record.as_slice();
        let v: Vec<&str> = s.split(";").map(|x| x.trim()).filter(|x| x.len() > 0).collect();

        let (raw_fen, raw_id) = (v[0], v[1]);

        drop(v);

        let id = String::from(&raw_id[4..raw_id.len()-1]);

        let mut fen_queue: VecDeque<&str> = raw_fen.split(" ").collect();

        let bm = fen_queue.pop_back().unwrap();

        fen_queue.pop_back();

        let fen: Vec<&str> = fen_queue.into();

        let fen = fen.join(" ");

        let board = Board::from_str(&fen).unwrap();

        let bm = ChessMove::from_san(&board, bm).unwrap();

        // println!("{:?} {:?} {:?}", fen, bm, id);
        puzzles.push(ChessPuzzle {
            bm,
            board,
            id
        });
    }
    Ok(puzzles)
}

fn main() {
    // if let Err(err) = example() {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }

    let test_positions = puzzlify_epd().unwrap();

    let mut engine = Engine::new(6);

    let mut solved = 0;
    let start_time = Instant::now();

    let test_count = test_positions.len();

    for puzzle in test_positions {

        let best = engine.test_calculate(&puzzle.board);

        if best == puzzle.bm {
            color_print::cprintln!("{}, <green>PASS</green> {}", puzzle.id, start_time.elapsed().as_secs_f32());
            solved += 1;
        } else {
            color_print::cprintln!("{}: <red>FAIL</red> {}", puzzle.id, start_time.elapsed().as_secs_f32());
        }
    }

    println!("Completed tests: {} out of {}", solved, test_count);

}