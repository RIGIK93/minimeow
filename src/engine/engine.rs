use std::{sync::mpsc::{self, Receiver}, thread::{self, JoinHandle}, time::Instant};

use chess::{Board, ChessMove, Color, MoveGen};

use crate::engine::{alphabeta::{alpha_beta_max, alpha_beta_min, alphabeta}, evaluation::CP, transposition_table::TranspositionTable};

use super::{evaluation::{material_eval, LARGE_EVAL, SMALL_EVAL}, move_tree::MoveTree};

pub struct Engine {
    pub depth: u8,
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
            eval = alpha_beta_max(&t, tt, SMALL_EVAL, LARGE_EVAL, depth - 1);
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
            eval = alpha_beta_min(&t, tt, SMALL_EVAL, LARGE_EVAL, depth - 1);
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

    fn calculate(pos: &Board, depth: u8) -> Option<ChessMove> {
        let mut tt = TranspositionTable::new();
        let mut bm = ChessMove::default();
        
        for i in 1..depth {
            let tree: MoveTree = MoveTree::new_root_node( material_eval, pos.clone());
            let calc_start = Instant::now();

            let (best, eval) = Self::best_move(&tree, &mut tt, i);
            bm = best;

            let node_count = tree.get_node_count();

            println!("info nodes {} nps {} time {} pv {} cp {}", node_count, ((node_count as f64)/calc_start.elapsed().as_secs_f64()).round(), calc_start.elapsed().as_millis(), bm.to_string(), eval);
        }

        // if let Some(entry) = tt.get(pos) {
        //     return Some(entry.mv)
        // }
        if bm != ChessMove::default() {
            return Some(bm)
        }

        None
    }

    /// TODO: Thread intentionally panics, resolve smoother
    /// check whether legal moves are available at the start instead of at the end.
    pub fn start(&mut self, position: Board) {
        let (tx, rx) = mpsc::channel::<ChessMove>();
        self.rx = Some(rx);
        let depth = self.depth;
        self.handle = thread::spawn(move || {
            let mv = Engine::calculate(&position, depth).unwrap();
            tx.send(mv).unwrap();
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
