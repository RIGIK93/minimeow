use core::hash;
use std::{collections::HashMap, default, hash::Hash, str::FromStr};

use chess::{Board, ChessMove};

use super::evaluation::CP;

use nohash_hasher::IntMap;

/// Since alphabeta nodes can cut off, 
/// sometimes we only know the range of values that eval might take at selected depth,
/// So we need to store the state of what eval means for us:
/// Have we searched the entire node or did we make a cutoff and only know that if we search the tree
/// we encouter values either below or above stored in the entry eval?
pub enum BoundType {
    RecedesLowerBound,
    ExceedsUpperBound,
    Exact
}

pub struct SearchTableEntry {
    /// zobrist hash of a position
    pub hashed_position: u64, 
    /// depth to which the subtree to which this position belongs was searched
    pub depth: u8,
    /// best move
    pub mv: ChessMove,
    pub eval: CP,
    pub flag: BoundType
}

pub struct TranspositionTable(IntMap<u64, SearchTableEntry>);

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable(IntMap::default())
    }

    pub fn set(&mut self, board: &Board, depth: u8, mv: ChessMove, eval: CP, eval_type: BoundType) {
        let hashed_position = board.get_hash();
        self.0.insert(hashed_position, SearchTableEntry {
            hashed_position,
            depth,
            eval,
            mv,
            flag: eval_type
        });
    }

    pub fn get(&self, board: &Board) -> Option<&SearchTableEntry> {
        let hash = board.get_hash();
        match self.0.get(&hash) {
            // Collision detection
            Some(e) => {
                if e.hashed_position != hash {
                    return None
                }
                return Some(e)
            },
            None => return None
        }
    }

    pub fn set_if_deeper(&mut self, board: &Board, depth: u8, mv: ChessMove, eval: CP, eval_type: BoundType) {
        match self.get(board) {
            Some(e) => {
                if depth > e.depth {
                    self.set(board, depth, mv, eval, eval_type);
                }
            }
            None => self.set(board, depth, mv, eval, eval_type),
        }
    }

}

#[test]
fn hash_test() {
    let (b1, b2) = (Board::default(), Board::default());

    let b3 = b2.make_move_new(ChessMove::from_str("e2e4").unwrap());

    assert_eq!(b1.get_hash(), b2.get_hash());
    assert_ne!(b1.get_hash(), b3.get_hash());
    assert_ne!(b2.get_hash(), b3.get_hash());
}

