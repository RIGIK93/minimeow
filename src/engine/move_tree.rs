use std::{cell::RefCell, rc::Rc};

use chess::{Board, ChessMove, MoveGen};

use super::evaluation::{EvalFunc, CP};

#[derive(Clone)]
pub struct MoveTree {
    // pub mv: ChessMove,
    // status: MoveTreeStatus,
    // depth: u8,
    node_count: Rc<RefCell<u64>>,
    pub eval_func: EvalFunc,
    pub board: Board,
    // pub children: Vec<MoveTree>
}

impl MoveTree {
    pub fn new(mv: ChessMove, eval_func: EvalFunc, board: &Board) -> Self {
        Self {
            // mv,
            // depth: 0,
            eval_func: eval_func,
            // children: Vec::new(),
            board: board.make_move_new(mv),
            // status: Default::default()
            node_count: Rc::new(RefCell::new(0 as u64))
        }
    }

    fn gen_child(&self, mv: ChessMove) -> MoveTree {
        *self.node_count.borrow_mut() += 1;    

        Self {
            eval_func: self.eval_func,
            board: self.board.make_move_new(mv),
            node_count: self.node_count.clone()
        }
    }

    pub fn get_node_count(&self) -> u64 {
        *self.node_count.borrow()
    }

    pub fn gen_children(&self) -> Vec<MoveTree> {
        MoveGen::new_legal(&self.board)
        .map(|mv| self.gen_child(mv))
        .collect()
    }

    pub fn eval(&self) -> CP {
        return (self.eval_func)(&self.board)
    }
}

#[test]
fn tree_gen_test() {
    use crate::engine::{evaluation::material_eval, print_board::print_board};

    let board = Board::default();
    let mv = MoveGen::new_legal(&board).last().unwrap();
    let tree = MoveTree::new(mv, material_eval, &board);
    let children = tree.gen_children();
    print_board(&tree.board);
    for c in &children {
        print_board(&c.board);
    }
    println!("{}", children.len());
    assert_eq!(children.len(), 20);
}
