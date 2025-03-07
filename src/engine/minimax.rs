use chess::BoardStatus;
use super::{evaluation::{LARGE_EVAL, SMALL_EVAL}, move_tree::MoveTree};

#[allow(dead_code)]
pub fn minimax(tree: &MoveTree, depth: u8) -> f64 {
    match tree.board.side_to_move() {
        chess::Color::White => maxi(tree, depth),
        chess::Color::Black => mini(tree, depth)
    }
}

pub fn maxi(tree: &MoveTree, depth: u8) -> f64 {

    if depth == 0 {
        return tree.eval();
    }

    let mut max = SMALL_EVAL;
    let children = tree.gen_children();

    if children.len() == 0 {
        if tree.board.status() == BoardStatus::Checkmate {
            return SMALL_EVAL;
        }

        if tree.board.status() == BoardStatus::Stalemate {
            return 0.0;
        }
    }

    for child in children {
        let score = mini(&child, depth - 1);
        if score > max {
            max = score;
        }
    }

    return max;
}

pub fn mini(tree: &MoveTree, depth: u8) -> f64 {

    if depth == 0 {
        return tree.eval();
    }

    let mut min = LARGE_EVAL;
    let children = tree.gen_children();

    if children.len() == 0 {
        if children.len() == 0 {
            if tree.board.status() == BoardStatus::Checkmate {
                return LARGE_EVAL;
            }
    
            if tree.board.status() == BoardStatus::Stalemate {
                return 0.0;
            }
        }
    }

    for child in children {
        if child.board.status() == BoardStatus::Checkmate {
            return SMALL_EVAL;
        }

        let score = maxi(&child, depth - 1);
        if score < min {
            min = score;
        }
    }

    return min;
}

#[test]
fn mate_in_three() {
    use std::str::FromStr;
    use chess::{Board, ChessMove};
    use crate::engine::{evaluation::material_eval, print_board::print_board};

    let board: Board = Board::from_str("1B2k3/8/3q4/5Q2/8/8/4K3/8 w - - 0 1").unwrap();
    print_board(&board);

    let tree = MoveTree::new(ChessMove::from_str("b8d6").unwrap(), material_eval, &board);
    // let eval = mini(&tree, 5);
    let eval = minimax(&tree, 5);


    println!("----------");
    print_board(&board.make_move_new(ChessMove::from_str("b8d6").unwrap()));
    assert_eq!(eval, LARGE_EVAL);
}
