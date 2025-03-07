
pub mod evaluation;
pub mod engine;
pub mod print_board;
pub mod minimax;
pub mod move_tree;
pub mod uci;
pub mod alphabeta;

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


