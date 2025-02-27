use std::{borrow::Cow, collections::VecDeque, fs::File, io, ops::Deref, str::FromStr, sync::mpsc::{self, Receiver}, thread::{self, JoinHandle, Thread}};
use chess::{Board, BoardStatus, ChessMove, Color, Game, MoveGen, Piece, Square};
use ptree::{print_tree, write_tree, Style, TreeItem};

// enum MoveTreeStatus {
//     Default
// }

// impl Default for MoveTreeStatus {
//     fn default() -> Self {
//         Self::Default
//     }
// }

type EvalFunc = fn(&Board) -> f64;

#[derive(Clone)]
struct MoveTree {
    mv: ChessMove,
    // status: MoveTreeStatus,
    // depth: u8,
    eval_func: EvalFunc,
    board: Board,
    children: Vec<MoveTree>
}

impl MoveTree {
    fn new(mv: ChessMove, eval_func: EvalFunc, board: &Board) -> Self {
        Self {
            mv,
            // depth: 0,
            eval_func: eval_func,
            children: Vec::new(),
            board: board.make_move_new(mv)
            // status: Default::default()
        }
    }

    fn add_child(&mut self, mv: ChessMove) {
        self.children.push(MoveTree::new(mv, self.eval_func, &self.board));
    }

    fn gen_new_children(&mut self) {
        self.children = Vec::new();
        for mv in MoveGen::new_legal(&self.board) {
            self.add_child(mv);
        }
    }

    fn eval(&self) -> f64 {
        return (self.eval_func)(&self.board)
    }

    // fn gen_to_depth(depth: u8) {

    // }

    fn get_child(&self, mv: ChessMove) -> Option<&MoveTree> {
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

// fn mini(tree: &mut MoveTree, depth: u8) -> f64;

fn maxi(tree: &mut MoveTree, depth: u8) -> f64 {
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

fn mini(tree: &mut MoveTree, depth: u8) -> f64 {
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

const LARGE_EVAL: f64 = 1000000000 as f64;
const SMALL_EVAL: f64 = -LARGE_EVAL;

fn print_board(board: &Board) {
    let mut custom: String = String::new();

    for i in 0..64 {
        let sq = unsafe { Square::new(i) };
        let piece = board.piece_on(sq);

        let file: &str = match i % 8 {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => unreachable!(),
        };

        let rank = (i / 8 + 1) as u32;

        if file == "a" {
            custom.push_str(format!("{} ", rank).as_str());
        }

        if let Some(p) = piece {
            let mut piece = String::from(match p {
                Piece::Bishop => "B ",
                Piece::King => "K ",
                Piece::Knight => "N ",
                Piece::Pawn => "P ",
                Piece::Queen => "Q ",
                Piece::Rook => "R ",
            });

            piece = match board.color_on(sq).unwrap() {
                chess::Color::White => piece.to_uppercase(),
                chess::Color::Black => piece.to_lowercase(),
            };

            // custom.push_str(format!("{}{}.{},{} ", file, rank, color, piece).as_str());
            custom.push_str(&piece);
        } else {
            if rank % 2 == 0 {
                if (i % 8) % 2 == 0 {
                    custom.push_str("# ");
                } else {
                    custom.push_str("  ");
                }
            } else {
                if (i % 8) % 2 == 1 {
                    custom.push_str("# ");
                } else {
                    custom.push_str("  ");
                }
            }
        }

        // Add \ at the end of the file
        if file == "h" {
            custom.push_str("\n");
        }
    }

    custom.push_str("  a b c d e f g h");

    // let en = ChessTurnEngine::new(Setup::Custom(&*fc)).unwrap();
    // // println!("{}", en.display(DisplayOption::BoardView(ViewMode::FancyTui)));
    // en.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));
    println!("{}", custom);
}


fn default_eval(b: &Board) -> f64 {
    let mut sq = Square::A1;
    let mut eval: f64 = 0.0;

    loop {
        if let Some(p) = b.piece_on(sq) {
            let tmp: f64 = match p {
                Piece::Bishop => 3.0,
                Piece::Knight => 3.0,
                Piece::King => LARGE_EVAL,
                Piece::Pawn => 1.0,
                Piece::Queen => 9.0,
                Piece::Rook => 5.0,
            };

            match b.color_on(sq).unwrap() {
                Color::White => eval += tmp,
                Color::Black => eval -= tmp,
            }
        }

        // scan the board right until the last file, then up
        match sq.right() {
            Some(s) => sq = s,
            None => match sq.up() {
                Some(u) => sq = u.uright(),
                None => break,
            },
        }
    }

    eval
}

fn alphabeta(e: EvalFunc, board: &Board, mut a: f64, b: f64, depth: u8) -> f64 {

    if board.status() != BoardStatus::Ongoing {
        if board.status() == BoardStatus::Stalemate {
            return 0.0;
        }

        // handle checkmate
        return match board.side_to_move() {
            Color::White => SMALL_EVAL, // Black Mated
            Color::Black => LARGE_EVAL // White Mated
        }
    }

    if depth == 0 {
        return e(&board);
    } // Replace with quiesce search for horizon effect mitigation

    let mut best = SMALL_EVAL;
    for mv in MoveGen::new_legal(board) {
        // println!("{}", mv);
        let board: Board = board.make_move_new(mv);
        // print_board(&board); // Debug
        let score: f64 = -alphabeta(e, &board, -b, -a, depth - 1);

        if score > best {
            best = score;
            if score > a {
                a = score; // alpha acts like max in MiniMax
            }
        }
        if score >= b {
            return best; //  fail soft beta-cutoff, existing the loop here is also fine
        }
    }

    return best;
}

// alpha < beta
fn alphaBetaMax(evaluate: EvalFunc, board: &Board, mut alpha: f64, beta: f64, depthleft: u8) -> f64 {
    if depthleft == 0 || (board.status() != BoardStatus::Ongoing) {
        return evaluate(board);
    }

    let mut bestValue = SMALL_EVAL;
    for mv in MoveGen::new_legal(board) {
       let score = alphaBetaMin(evaluate, &board.make_move_new(mv), alpha, beta, depthleft - 1);
       if score > bestValue {
          bestValue = alpha;
          if score > alpha {
             alpha = score; // alpha acts like max in MiniMax
          }
       }
       if score >= beta {
          return score;   // fail soft beta-cutoff
       }
    }
    return bestValue;
 }

fn alphaBetaMin(evaluate: EvalFunc, board: &Board, alpha: f64, mut beta: f64, depthleft: u8) -> f64 {
    if depthleft == 0 || (board.status() != BoardStatus::Ongoing) {
        return -evaluate(board);
    }

    let mut bestValue = LARGE_EVAL;
    for mv in MoveGen::new_legal(board) {
       let score = alphaBetaMax(evaluate, &board.make_move_new(mv), alpha, beta, depthleft - 1 );
       if score < bestValue {
          bestValue = score;
          if score < beta {
             beta = score; // beta acts like min in MiniMax
          }
       }
       if score <= alpha {
          return score; // fail soft alpha-cutoff, break can also be used here
       }
    }
    return bestValue;
 }

// const DEPTH: u8 = 7;

// fn main() {
//     // let board = Board::default();
//     // let mut moves = MoveGen::new_legal(&board);
//     // println!("{}", board.to_string());
//     // let str = board.to_string();
//     // board = board.make_move_new(moves.nth(6).unwrap());
//     // print_board(&board);
//     // println!("{}", default_eval(&board));
//     // let mut game = Game::new();
//     // let mut counter = 0;
//     let board: Board = Board::from_str("1B2k3/8/3q4/5Q2/8/8/4K3/8 w - - 0 1").unwrap();
//     print_board(&board);
//     let mut best = SMALL_EVAL;
//     let mut best_mv = ChessMove::default();
//     // f5b5
//     // b8d6
//     // let moves = MoveGen::new_legal(&board);
//     let moves = vec![ChessMove::from_str("f5b5").unwrap(), ChessMove::from_str("b8d6").unwrap()];
//     for mv in moves {
//         let board = board.make_move_new(mv);
//         let tmp = alphabeta(default_eval, &board, SMALL_EVAL, LARGE_EVAL, DEPTH);
//         // let tmp = alphaBetaMax(default_eval, &board, SMALL_EVAL, LARGE_EVAL, DEPTH);
//         println!("{}: {}", mv.to_string(), tmp);
//         if tmp > best {
//             best = tmp;
//             best_mv = mv;
//         }
//     }
//     // game.make_move(best_mv);
//     println!("{}", best_mv.to_string());
//     println!("----------");
//     print_board(&board.make_move_new(best_mv));
//     // println!("-------");
//     // print_board(&Board::default());
//     // for i in 0..=63 {
//     //     println!("{}: {}", i, (i / 8 + 1) as u32)
//     // }
//     // while game.result().is_none() {
//     //     // println!("{}", counter);
//     //     let board = game.current_position();
//     //     match game.side_to_move() {
//     //         Color::White => {
//     //             let mut best = SMALL_EVAL;
//     //             let mut best_mv = ChessMove::default();
//     //             for mv in MoveGen::new_legal(&board) {
//     //                 let board = board.make_move_new(mv);
//     //                 let tmp = alphabeta(default_eval, &board, LARGE_EVAL, SMALL_EVAL, DEPTH);
//     //                 if tmp > best {
//     //                     best = tmp;
//     //                     best_mv = mv;
//     //                 }
//     //             }
//     //             game.make_move(best_mv);
//     //         },
//     //         Color::Black => {
//     //             let mut best = LARGE_EVAL;
//     //             let mut best_mv = ChessMove::default();
//     //             for mv in MoveGen::new_legal(&board) {
//     //                 let board = board.make_move_new(mv);
//     //                 let tmp = alphabeta(default_eval, &board, SMALL_EVAL, LARGE_EVAL, DEPTH);
//     //                 if tmp < best {
//     //                     best = tmp;
//     //                     best_mv = mv;
//     //                 }
//     //             }
//     //             game.make_move(best_mv);
//     //         }
//     //     }
//     //     print_board(&board);
//     // }
//     // let en = ChessTurnEngine::new(Setup::Normal).unwrap();
//     // en.display_on_screen(DisplayOption::BoardView(ViewMode::FancyTui));
// }


fn uci_set_position(game: &mut Game, args: &mut VecDeque<&str>) {

    let mut fen = String::new();
    let mut moves: Vec<ChessMove> = Vec::new();
    let mut moves_started = false; // becomes true when "args" reach "move" word

    loop {
        if let Some(fen_piece) = args.pop_front() {
            if fen_piece == "moves" {
                moves_started = true;
                break;
            }

            fen.push_str(fen_piece);
        } else {
            break;
        }
    }

    while let Some(mv) = args.pop_front() {
        moves.push(ChessMove::from_str(mv).unwrap());
    }

    if fen == "startpos" {
        *game = Game::new_with_board(Board::default());
    } else {
        *game = Game::new_with_board(Board::from_str(&fen).unwrap());
    }

    for mv in moves {
        game.make_move(mv);
    }
}



struct Engine {
    depth: u8,
    // bv: Option<ChessMove>,
    handle: JoinHandle<()>,
    rx: Option<Receiver<ChessMove>>
}

impl Engine {
    fn new(depth: u8) -> Engine {
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

    fn calculate(pos: &Board, depth: u8) -> Option<ChessMove> {
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
    fn start(&mut self, position: Board) {
        let (tx, rx) = mpsc::channel::<ChessMove>();
        self.rx = Some(rx);
        let depth = self.depth;
        self.handle = thread::spawn(move || {
            let mv = Engine::calculate(&position, depth).unwrap();
            tx.send(mv).unwrap();
        });
    }

    fn stop(&self) -> ChessMove {
        // could use try_recv when iterative deepening is implemented

        // self.bv
        self.rx.as_ref().unwrap().recv().unwrap()
    }
}

fn uci_go(game: &mut Game, engine: &mut Engine, args: &mut VecDeque<&str>) -> Option<ChessMove> {
    if args.pop_front().unwrap_or_default() == "infinite" {
        engine.start(game.current_position());
    }

    None
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let mut game = Game::new();
    let mut engine = Engine::new(3);


    while buffer != "quit\n" {
        buffer = String::new();
        stdin.read_line(&mut buffer)?;
        let mut tokens: VecDeque<&str> = buffer.trim().split_whitespace().collect();
        while tokens.len() != 0 {
            match tokens.pop_front().unwrap() {
                "uci" => {println!("id name MiniMeow\nid author RIGIK93\nuciok")},
                "isready" => {println!("readyok")},
                "quit" => return Ok(()),
                "ucinewgame" => {game = Game::new()},
                "position" => {uci_set_position(&mut game, &mut tokens);},
                "go" => {uci_go(&mut game, &mut engine, &mut tokens);},
                "stop" => {println!("bestmove {}", engine.stop().to_string());}
                "print" => {print_board(&game.current_position());},
                _ => {}
            }
        }
    }

    Ok(())
}

#[test]
#[should_panic]
fn panic_test() {
    panic!("I be panicking!");
}

#[test]
fn tree_gen_test() {
    let board = Board::default();
    let mv = MoveGen::new_legal(&board).last().unwrap();
    let mut tree = MoveTree::new(mv, default_eval, &board);
    tree.gen_new_children();
    print_board(&tree.board);
    for c in &tree.children {
        print_board(&c.board);
    }
    println!("{}", tree.children.len());
    assert_eq!(tree.children.len(), 20);
}

#[test]
fn basic_queen_taking_test() {
    let board: Board = Board::from_str("1B2k3/8/3q4/5Q2/8/8/4K3/8 w - - 0 1").unwrap();
    print_board(&board);

    let mut best = SMALL_EVAL;
    let mut best_mv = ChessMove::default();

    // f5b5
    // b8d6
    let moves = vec![ChessMove::from_str("f5b5").unwrap(), ChessMove::from_str("b8d6").unwrap()];
    for mv in moves {
        let mut tree = MoveTree::new(mv, default_eval, &board);
        let tmp = maxi(&mut tree, 3);

        println!("{}: {}", mv.to_string(), tmp);
        write_tree(&tree, File::create(&mv.to_string()).unwrap()).unwrap();

        if tmp > best {
            best = tmp;
            best_mv = mv;
        }
    }
    // game.make_move(best_mv);
    assert_eq!(best_mv.to_string(), "b8d6");
    println!("----------");
    print_board(&board.make_move_new(best_mv));
}

#[test]
fn engine_test() {
    let mut eng = Engine::new(3);

    eng.start(Board::default());

    let mv = eng.stop();

}
