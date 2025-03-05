use std::{borrow::Cow, io};

use chess::{Board, ChessMove, MoveGen};
use ptree::{Style, TreeItem};

use super::evaluation::EvalFunc;

#[derive(Clone)]
pub struct MoveTree {
    pub mv: ChessMove,
    // status: MoveTreeStatus,
    // depth: u8,
    pub eval_func: EvalFunc,
    pub board: Board,
    pub children: Vec<MoveTree>
}

impl MoveTree {
    pub fn new(mv: ChessMove, eval_func: EvalFunc, board: &Board) -> Self {
        Self {
            mv,
            // depth: 0,
            eval_func: eval_func,
            children: Vec::new(),
            board: board.make_move_new(mv)
            // status: Default::default()
        }
    }

    pub fn add_child(&mut self, mv: ChessMove) {
        self.children.push(MoveTree::new(mv, self.eval_func, &self.board));
    }

    pub fn gen_new_children(&mut self) {
        self.children = Vec::new();
        for mv in MoveGen::new_legal(&self.board) {
            self.add_child(mv);
        }
    }

    pub fn eval(&self) -> f64 {
        return (self.eval_func)(&self.board)
    }

    // fn gen_to_depth(depth: u8) {

    // }

    pub fn get_child(&self, mv: ChessMove) -> Option<&MoveTree> {
        for child in &self.children {
            if child.mv == mv {
                return Some(child);
            }
        }
        None
    }

    // fn max(&mut self) -> Option<&mut MoveTree> {
    //     if self.children.len() < 1 {
    //         return None;
    //     }

    //     let mut selected = &mut self.children[0];
    //     let mut selected_eval = selected.eval();

    //     for child in &mut self.children[1..] {

    //     }
        
    //     return Some(&mut selected);
    // }
}

impl TreeItem for MoveTree {
    type Child = Self;
    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        write!(f, "{}", style.paint(format!("{} ({})", self.mv.to_string(), self.eval())))
    }
    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(self.children.clone())
    }
}