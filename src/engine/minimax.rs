use super::{evaluation::{LARGE_EVAL, SMALL_EVAL}, move_tree::MoveTree};


pub fn maxi(tree: &mut MoveTree, depth: u8) -> f64 {
    if depth == 0 {
        return tree.eval();
    }

    let mut max = SMALL_EVAL;
    tree.gen_new_children();
    
    if tree.children.len() == 0 {
        return tree.eval();
    }

    for child in &mut tree.children {
        let score = mini(child, depth - 1);
        if score > max {
            max = score;
        }
    }

    return max;
}

pub fn mini(tree: &mut MoveTree, depth: u8) -> f64 {
    if depth == 0 {
        return tree.eval();
    }

    let mut min = LARGE_EVAL;
    tree.gen_new_children();

    if tree.children.len() == 0 {
        return tree.eval();
    }

    for child in &mut tree.children {
        let score = maxi(child, depth - 1);
        if score < min {
            min = score;
        }
    }

    return min;
}