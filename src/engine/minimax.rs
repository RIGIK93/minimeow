use super::{evaluation::{LARGE_EVAL, SMALL_EVAL}, move_tree::MoveTree};


pub fn maxi(tree: &MoveTree, depth: u8) -> f64 {
    if depth == 0 {
        return tree.eval();
    }

    let mut max = SMALL_EVAL;
    let mut children = tree.gen_children();
    
    if children.len() == 0 {
        return tree.eval();
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
        return tree.eval();
    }

    for child in children {
        let score = maxi(&child, depth - 1);
        if score < min {
            min = score;
        }
    }

    return min;
}