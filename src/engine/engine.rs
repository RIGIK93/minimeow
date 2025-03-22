use std::{sync::mpsc::{self, Receiver}, thread::{self, JoinHandle}, time::Instant};

use chess::{Board, ChessMove, Color};

use crate::engine::{alphabeta::{alpha_beta_max, alpha_beta_min, MAX_NODES}, evaluation::CP, transposition_table::TranspositionTable};

use super::{evaluation::{material_eval, LARGE_EVAL, SMALL_EVAL}, move_tree::MoveTree};

pub struct Engine {
    pub depth: u8,
    // pub nodes: u64,
    // bv: Option<ChessMove>,
    pub handle: JoinHandle<()>,
    pub rx: Option<Receiver<ChessMove>>
}

impl Engine {
    pub fn new(depth: u8) -> Engine {
        Engine {
            depth: depth,
            handle: thread::spawn(|| {}),
            rx: None,
            // nodes: 150000000 // hard cut off at 15 seconds for time considerations. Some positions go deep really fast, so a cutoff is needed. Since approximately 10,000,000 positions are searched per second, we add a cut of at 150 million
        }
    }

    // pub fn get_options() {
    //     todo!()
    // }

    // TODO
    // pub fn set_option() {
    //     todo!()
    // }

    fn best_move_max(tree: &MoveTree, tt: &mut TranspositionTable, depth: u8) -> (ChessMove, CP) {
        let mut max = SMALL_EVAL;
        let mut bm = ChessMove::default();

        let children = tree.gen_children();

        let mut eval;
        for t in children {
            eval = alpha_beta_min(&t, tt, SMALL_EVAL, LARGE_EVAL, depth - 1);
            if eval >= max {
                max = eval;
                bm = t.mv;
            }
        }

        (bm, max)
    }

    fn best_move_min(tree: &MoveTree, tt: &mut TranspositionTable, depth: u8) -> (ChessMove, CP) {
        let mut min = LARGE_EVAL;
        let mut bm = ChessMove::default();

        let children = tree.gen_children();

        let mut eval;
        for t in children {
            eval = alpha_beta_max(&t, tt, SMALL_EVAL, LARGE_EVAL, depth - 1);
            if eval <= min {
                min = eval;
                bm = t.mv;
            }
        }

        (bm, min)
    }

    fn best_move(tree: &MoveTree, tt: &mut TranspositionTable, depth: u8) -> (ChessMove, CP) {
        match tree.board.side_to_move() {
            Color::White => Self::best_move_max(tree, tt, depth),
            Color::Black => Self::best_move_min(tree, tt, depth)
        }
    }

    fn calculate(tx: std::sync::mpsc::Sender<ChessMove>, pos: &Board, depth: u8) {
        let mut tt = TranspositionTable::new();
        let mut bm = ChessMove::default();
        
        for i in 1..depth {
            let tree: MoveTree = MoveTree::new_root_node( material_eval, pos.clone());
            let calc_start = Instant::now();

            let (best, eval) = Self::best_move(&tree, &mut tt, i);

            let node_count = tree.get_node_count();

            if node_count >= MAX_NODES {
                break;
            }

            bm = best;
            tx.send(bm).unwrap();
            println!("info depth {} nodes {} nps {} time {} pv {} score cp {}", i, node_count, ((node_count as f64)/calc_start.elapsed().as_secs_f64()).round(), calc_start.elapsed().as_millis(), bm.to_string(), eval);
        }

        println!("bestmove {}", bm.to_string())

        // if let Some(entry) = tt.get(pos) {
        //     return Some(entry.mv)
        // }
        // if bm != ChessMove::default() {
        //     return Some(bm)
        // }

        // None
    }

    pub fn test_calculate(&self, pos: &Board) -> ChessMove {
        let mut tt = TranspositionTable::new();
        let mut bm = ChessMove::default();
        
        for i in 1..self.depth {
            let tree: MoveTree = MoveTree::new_root_node( material_eval, pos.clone());
            let calc_start = Instant::now();

            let (best, eval) = Self::best_move(&tree, &mut tt, i);
            bm = best;

            let node_count = tree.get_node_count();

            println!("info depth {} nodes {} nps {} time {} pv {} score cp {}", i, node_count, ((node_count as f64)/calc_start.elapsed().as_secs_f64()).round(), calc_start.elapsed().as_millis(), bm.to_string(), eval);
        }

        // println!("bestmove {}", bm.to_string())

        // if let Some(entry) = tt.get(pos) {
        //     return Some(entry.mv)
        // }
        // if bm != ChessMove::default() {
        //     return Some(bm)
        // }

        // None

        bm
    }

    /// TODO: Thread intentionally panics, resolve smoother
    /// check whether legal moves are available at the start instead of at the end.
    pub fn start(&mut self, position: Board) {
        let (tx, rx) = mpsc::channel::<ChessMove>();
        self.rx = Some(rx);
        let depth = self.depth;
        self.handle = thread::spawn(move || {
            Engine::calculate(tx, &position, depth);
        });
    }

    pub fn stop(&self) -> ChessMove {
        // could use try_recv when iterative deepening is implemented

        match self.rx.as_ref().unwrap().recv() {
            Ok(mv) => return mv,
            Err(e) => panic!("{}", e)
        }
    }
}


#[test]
fn engine_test() {
    let mut eng = Engine::new(3);

    eng.start(Board::default());

    eng.stop();
}

#[test]
fn fen_test() {
    use super::super::engine::alphabeta::alphabeta;
    use std::str::FromStr;

    let board = Board::from_str("2rqkbnr/pppbp1p1/5p1p/3Q4/2B1P3/8/PPP2PPP/RNB1K1NR b KQk - 3 7").unwrap();
    println!("side to move: {}", match board.side_to_move() {
        Color::Black => "black",
        Color::White => "white"
    });
    let best = ChessMove::from_str("e7e6").unwrap();
    let worst = ChessMove::from_str("h8h7").unwrap();

    let mut tt1 = TranspositionTable::new();
    let mut tt2 = TranspositionTable::new();
    let bt = MoveTree::new(best, material_eval, &board);
    let wt = MoveTree::new(worst, material_eval, &board);

    // let engine = Engine::new(8);

    let b = alphabeta(&bt, &mut tt1, 5);
    let w = alphabeta(&wt, &mut tt2, 5);

    println!("best move: {} cp", b);
    println!("worst move: {} cp", w);

    assert!(w > b); 
}
