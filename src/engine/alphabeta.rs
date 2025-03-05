pub fn alphabeta(e: EvalFunc, board: &Board, mut a: f64, b: f64, depth: u8) -> f64 {

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
pub fn alphaBetaMax(evaluate: EvalFunc, board: &Board, mut alpha: f64, beta: f64, depthleft: u8) -> f64 {
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

pub fn alphaBetaMin(evaluate: EvalFunc, board: &Board, alpha: f64, mut beta: f64, depthleft: u8) -> f64 {
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