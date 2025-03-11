// pub fn alphabeta(e: EvalFunc, board: &Board, mut a: f64, b: f64, depth: u8) -> f64 {

//     if board.status() != BoardStatus::Ongoing {
//         if board.status() == BoardStatus::Stalemate {
//             return 0.0;
//         }

//         // handle checkmate
//         return match board.side_to_move() {
//             Color::White => SMALL_EVAL, // Black Mated
//             Color::Black => LARGE_EVAL // White Mated
//         }
//     }

//     if depth == 0 {
//         return e(&board);
//     } // Replace with quiesce search for horizon effect mitigation

//     let mut best = SMALL_EVAL;
//     for mv in MoveGen::new_legal(board) {
//         // println!("{}", mv);
//         let board: Board = board.make_move_new(mv);
//         // print_board(&board); // Debug
//         let score: f64 = -alphabeta(e, &board, -b, -a, depth - 1);

//         if score > best {
//             best = score;
//             if score > a {
//                 a = score; // alpha acts like max in MiniMax
//             }
//         }
//         if score >= b {
//             return best; //  fail soft beta-cutoff, existing the loop here is also fine
//         }
//     }

//     return best;
// }

use chess::BoardStatus;

use super::{
    evaluation::{CP, LARGE_EVAL, SMALL_EVAL},
    move_tree::MoveTree,
};

#[allow(dead_code)]
pub fn alphabeta(tree: &MoveTree, depth: u8) -> CP {
   match tree.board.side_to_move() {
       chess::Color::White => alpha_beta_max(tree, SMALL_EVAL, LARGE_EVAL, depth),
       chess::Color::Black => alpha_beta_min(tree, SMALL_EVAL, LARGE_EVAL, depth)
   }
}

// alpha < beta
pub fn alpha_beta_max(tree: &MoveTree, mut lower: CP, upper: CP, depth: u8) -> CP {
    if depth == 0 {
        return tree.eval();
    }

    let mut max = SMALL_EVAL;
    let children = tree.gen_children();

    // Mate detection
    if children.len() == 0 {
        match tree.board.status() {
            BoardStatus::Checkmate => return SMALL_EVAL,
            BoardStatus::Stalemate => return 0,
            BoardStatus::Ongoing => unreachable!(),
        }
    }

    for child in children {
        let score = alpha_beta_min(&child, lower, upper, depth - 1);

        if score > max {
            max = score;
            if score > lower {
                lower = score;
            }
        }

        if score >= upper {
            return score; // fail soft beta-cutoff
        }
    }

    return max;
}

pub fn alpha_beta_min(tree: &MoveTree, lower: CP, mut upper: CP, depth: u8) -> CP {
    if depth == 0 {
        return tree.eval();
    }

    let mut min = LARGE_EVAL;
    let children = tree.gen_children();

    // Mate detection
    if children.len() == 0 {
        match tree.board.status() {
            BoardStatus::Checkmate => return LARGE_EVAL,
            BoardStatus::Stalemate => return 0,
            BoardStatus::Ongoing => unreachable!(),
        }
    }

    for child in children {
        let score = alpha_beta_max(&child, lower, upper, depth - 1);
        if score < min {
            min = score;
            if score < upper {
                upper = score;
            }
        }

        if score <= lower {
            return score; // fail soft alpha-cutoff, break can also be used here
        }
    }

    return min;
}

// A great video of visual alphabeta procedure: https://www.youtube.com/watch?v=l-hh51ncgDI
// Code taken from: https://www.chessprogramming.org/Alpha-Beta#Max_versus_Min

#[test]
fn mate_in_three() {
    use std::str::FromStr;
    use chess::{Board, ChessMove};
    use crate::engine::{evaluation::material_eval, print_board::print_board};

    let board: Board = Board::from_str("1B2k3/8/3q4/5Q2/8/8/4K3/8 w - - 0 1").unwrap();
    print_board(&board);

    let tree = MoveTree::new(ChessMove::from_str("b8d6").unwrap(), material_eval, &board);
    // let eval = mini(&tree, 5);
    let eval = alphabeta(&tree, 5);


    println!("----------");
    print_board(&board.make_move_new(ChessMove::from_str("b8d6").unwrap()));
    assert!(eval > 30);
}
