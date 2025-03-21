use std::{hash::Hash, vec};

use chess::{Board, ChessMove};

use super::evaluation::CP;

use nohash_hasher::IntMap;

/// Since alphabeta nodes can cut off, 
/// sometimes we only know the range of values that eval might take at selected depth,
/// So we need to store the state of what eval means for us:
/// Have we searched the entire node or did we make a cutoff and only know that if we search the tree
/// we encouter values either below or above stored in the entry eval?
#[derive(Clone, Copy)]
pub enum BoundType {
    RecedesLowerBound,
    ExceedsUpperBound,
    Exact
}

#[derive(Clone, Copy)]
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

pub struct TranspositionTable(ZobristMap);

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable(ZobristMap::new())
    }

    pub fn set(&mut self, board: &Board, depth: u8, mv: ChessMove, eval: CP, flag: BoundType) {
        self.0.set(board, depth, mv, eval, flag);
    }

    pub fn get(&self, board: &Board) -> Option<&SearchTableEntry> {
        self.0.get(board)
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


const MAP_SIZE: usize = 16777216; // 2^24, roughly 16 mb
pub struct ZobristMap {
    arr: Box<[Option<SearchTableEntry>]>
}

impl ZobristMap {
    pub fn new() -> Self {
        ZobristMap { arr: vec![None; MAP_SIZE].into_boxed_slice() }
    }

    fn index(b: &Board) -> usize {
        (b.get_hash() as usize) % MAP_SIZE
    }

    pub fn get(&self, b: &Board) -> Option<&SearchTableEntry> {
        match &self.arr[Self::index(b)] {
            // Collision detection
            Some(e) => {
                if e.hashed_position != b.get_hash() {
                    return None
                }
                return Some(e)
            },
            None => return None
        }
    }

    pub fn set(&mut self, board: &Board, depth: u8, mv: ChessMove, eval: CP, flag: BoundType) {
        let hashed_position = board.get_hash();
        self.arr[Self::index(board)] = Some(SearchTableEntry { hashed_position, depth, mv, eval, flag});
    }
}

#[test]
fn hash_test() {
    use std::str::FromStr;

    let (b1, b2) = (Board::default(), Board::default());

    let b3 = b2.make_move_new(ChessMove::from_str("e2e4").unwrap());

    assert_eq!(b1.get_hash(), b2.get_hash());
    assert_ne!(b1.get_hash(), b3.get_hash());
    assert_ne!(b2.get_hash(), b3.get_hash());
}

#[test]
fn NoOverflowTest() {
    use std::str::FromStr;

    let (b1, b2) = (Board::default(), Board::default());

    let b3 = b2.make_move_new(ChessMove::from_str("e2e4").unwrap());

    let mut map = ZobristMap::new();   
}

#[test]
fn ZobristMap_Test() {
    use std::str::FromStr;

    let (b1, b2) = (Board::default(), Board::default());

    let b3 = b2.make_move_new(ChessMove::from_str("e2e4").unwrap());

    let mut map = ZobristMap::new();

    map.set(&b1, 1, Default::default(), Default::default(), BoundType::Exact);
    map.set(&b2, 2, Default::default(), Default::default(), BoundType::ExceedsUpperBound);
    map.set(&b3, 3, Default::default(), Default::default(), BoundType::RecedesLowerBound);


    assert_eq!(map.get(&b1).unwrap().depth, map.get(&b2).unwrap().depth);
    assert_ne!(map.get(&b2).unwrap().depth, map.get(&b3).unwrap().depth);
    assert_ne!(map.get(&b2).unwrap().depth, map.get(&b3).unwrap().depth);
}
