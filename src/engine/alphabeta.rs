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
    move_tree::MoveTree, transposition_table::TranspositionTable, 
    transposition_table::BoundType
};

#[allow(dead_code)]
pub fn alphabeta(tree: &MoveTree, tt: &mut TranspositionTable, depth: u8) -> CP {
   match tree.board.side_to_move() {
       chess::Color::White => alpha_beta_max(tree, tt, SMALL_EVAL, LARGE_EVAL, depth),
       chess::Color::Black => alpha_beta_min(tree, tt, SMALL_EVAL, LARGE_EVAL, depth)
   }
}

pub const MAX_NODES: u64 = 80_000_000;

// alpha < beta
pub fn alpha_beta_max(tree: &MoveTree, tt: &mut TranspositionTable, mut lower: CP, upper: CP, depth: u8) -> CP {
    let mut max = SMALL_EVAL;

    if let Some(entry) = tt.get(&tree.board) {
        if entry.depth > depth {
            match entry.flag {
                BoundType::Exact => {
                    return entry.eval;
                },
                BoundType::ExceedsUpperBound => {
                    if entry.eval > lower {
                        lower = entry.eval;
                    }

                    if lower >= upper {
                        return entry.eval;
                    }

                    // safe generation with a move legality needed in case of a hash collision
                    // note, the docs say that the legality check is pretty slow, so
                    // benchmarking is required
                    if let Some(t) = &tree.safe_gen_child(entry.mv) {
                        max = alpha_beta_min(t, tt, lower, upper, depth);

                        if max >= upper {
                            return max
                        }
                    }
                }
                BoundType::RecedesLowerBound => {/*hash collision */},
            }
        }
    }

    if depth == 0 || tree.get_node_count() > MAX_NODES {
        return tree.eval();
    }

    let children = tree.gen_children();

    // Mate detection
    if children.len() == 0 {
        match tree.board.status() {
            BoardStatus::Checkmate => return SMALL_EVAL,
            BoardStatus::Stalemate => return 0,
            BoardStatus::Ongoing => unreachable!(),
        }
    }

    let mut pv= children[0].clone();
    for child in children {
        let score = alpha_beta_min(&child, tt, lower, upper, depth - 1);

        if score > max {
            max = score;
            pv = child;
            if score > lower {
                lower = score;
            }
        }

        if score >= upper {
            tt.set_if_deeper(&pv.board, depth, pv.mv, score, BoundType::ExceedsUpperBound);
            return score; // fail soft beta-cutoff
        }
    }

    tt.set_if_deeper(&pv.board, depth, pv.mv, max, BoundType::Exact);
    return max;
}

pub fn alpha_beta_min(tree: &MoveTree, tt: &mut TranspositionTable, lower: CP, mut upper: CP, depth: u8) -> CP {
    let mut min = LARGE_EVAL;

    if let Some(entry) = tt.get(&tree.board) {
        if entry.depth > depth {
            match entry.flag {
                BoundType::Exact => {
                    return entry.eval;
                },
                BoundType::RecedesLowerBound => {
                    if entry.eval < upper {
                        upper = entry.eval;
                    }

                    if lower >= upper {
                        return entry.eval;
                    }

                    // safe generation with a move legality needed in case of a hash collision
                    // note, the docs say that the legality check is pretty slow, so
                    // benchmarking is required
                    if let Some(t) = &tree.safe_gen_child(entry.mv) {
                        min = alpha_beta_min(t, tt, lower, upper, depth);

                        if min <= lower {
                            return min
                        }
                    }
                }
                BoundType::ExceedsUpperBound => {/*hash collision */}, // Collision
            }
        }
    }

    if depth == 0 || tree.get_node_count() > MAX_NODES {
        return tree.eval();
    }

    let children = tree.gen_children();

    // Mate detection
    if children.len() == 0 {
        match tree.board.status() {
            BoardStatus::Checkmate => return LARGE_EVAL,
            BoardStatus::Stalemate => return 0,
            BoardStatus::Ongoing => unreachable!(),
        }
    }

    let mut pv= children[0].clone();
    for child in children {
        let score = alpha_beta_max(&child, tt, lower, upper, depth - 1);
        if score < min {
            min = score;
            pv = child;
            if score < upper {
                upper = score;
            }
        }

        if score <= lower {
            tt.set(&pv.board, depth, pv.mv, score, BoundType::RecedesLowerBound);
            return score; // fail soft alpha-cutoff, break can also be used here
        }
    }

    tt.set(&pv.board, depth, pv.mv, min, BoundType::Exact);
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
    let mut tt = TranspositionTable::new();
    // let eval = mini(&tree, 5);
    let eval = alphabeta(&tree, &mut tt, 5);


    println!("----------");
    print_board(&board.make_move_new(ChessMove::from_str("b8d6").unwrap()));
    assert!(eval > 30);
}

#[test]
// ensures that the sign of evaluation does not alternate with depth
fn sign_consistency() {
    use std::str::FromStr;
    use chess::{Board, ChessMove};
    use crate::engine::evaluation::material_eval;

    let board: Board = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w kq - 0 1").unwrap();

    let tree = MoveTree::new(ChessMove::from_str("e2e4").unwrap(), material_eval, &board);

    let mut tt = TranspositionTable::new();

    let eval_odd = alphabeta(&tree, &mut tt, 5);

    let tree = MoveTree::new(ChessMove::from_str("e2e4").unwrap(), material_eval, &board);

    let mut tt = TranspositionTable::new();

    let eval_even = alphabeta(&tree, &mut tt, 4);

    assert_eq!(eval_even.signum(), eval_odd.signum());
}
