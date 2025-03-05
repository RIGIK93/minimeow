use std::{sync::mpsc::{self, Receiver}, thread::{self, JoinHandle}};

use chess::{Board, ChessMove, Color, MoveGen};

use super::{evaluation::{default_eval, LARGE_EVAL, SMALL_EVAL}, minimax::{maxi, mini}, move_tree::MoveTree};

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

    fn get_options() {
        todo!()
    }

    // TODO
    fn set_option() {
        todo!()
    }

    pub fn calculate(pos: &Board, depth: u8) -> Option<ChessMove> {
        let moves: Vec<ChessMove> = MoveGen::new_legal(pos).collect();

        if moves.len() == 0 {
            return None
        }

        let mut best = ChessMove::default();

        match pos.side_to_move() {
            Color::White => {
                let mut best_eval = SMALL_EVAL;
                for mv in moves {
                    let mut tree: MoveTree = MoveTree::new(mv, default_eval, pos);

                    let current_eval = maxi(&mut tree, depth);
                    if current_eval > best_eval {
                        best_eval = current_eval;
                        best = mv;
                    }
                }
            }

            Color::Black => {
                let mut best_eval = LARGE_EVAL;
                for mv in moves {
                    let mut tree: MoveTree = MoveTree::new(mv, default_eval, pos);

                    let current_eval = mini(&mut tree, depth);
                    if current_eval < best_eval {
                        best_eval = current_eval;
                        best = mv;
                    }
                }
            }
        }

        Some(best)
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

        // self.bv
        self.rx.as_ref().unwrap().recv().unwrap()
    }
}