use crate::board::*;
use crossbeam;
use std::cmp::max;
use std::cmp::min;
use std::sync::Mutex;
use std::time::Instant;

struct EvaluatedBoard {
    board: Board,
    eval: i32,
}

impl EvaluatedBoard {
    fn new(board: Board, eval: i32) -> EvaluatedBoard {
        EvaluatedBoard { board, eval }
    }
}

impl Copy for EvaluatedBoard {}

impl Clone for EvaluatedBoard {
    fn clone(&self) -> EvaluatedBoard {
        *self
    }
}

pub fn decision_minmax(board: &Board, player: usize) -> (Color, usize) {
    const THREAD_AMOUNT: usize = 4;
    let mut max_depth;
    let now = Instant::now();

    let cpt = Mutex::new(0);
    let cpt_cut = Mutex::new(0);
    let values = Mutex::new([[0; 16]; 2]);

    let indexs_per_threads = Mutex::new([(); THREAD_AMOUNT].map(|_| Vec::new()));
    let mut thread_index = 0;
    let mut moves_amount = 0;

    for color in 0..2 {
        for hole in 0..16 {
            if board.is_this_move_possible(player, hole, Color::from_integer(color)) {
                indexs_per_threads.lock().unwrap()[thread_index].push((color, hole));
                thread_index = (thread_index + 1) % THREAD_AMOUNT;
                moves_amount += 1;
            } else {
                values.lock().unwrap()[color][hole] = i32::MIN;
            }
        }
    }

    if moves_amount > 12 {
        max_depth = 9;
    } else if moves_amount > 1 {
        max_depth = 10;
    } else {
        max_depth = 0;
    }
    
    println!("Starting turn...");
    while now.elapsed().as_secs_f32() <= 0.5 {
        crossbeam::scope(|scope| {
            for ipt in 0..THREAD_AMOUNT {
                let cpt = &cpt;
                let cpt_cut = &cpt_cut;
                let indexs_per_threads = &indexs_per_threads;
                let values = &values;
                
                scope.spawn(move |_| {
                    let ipt_len = indexs_per_threads.lock().unwrap()[ipt].len();
                    for t in 0..ipt_len {
                        let color = indexs_per_threads.lock().unwrap()[ipt][t].0;
                        let hole = indexs_per_threads.lock().unwrap()[ipt][t].1;
                        let mut new_board = *board;

                        new_board.play_move(player, hole, Color::from_integer(color));
                        let mut local_cpt = 0;
                        let mut local_cpt_cut = 0;
                        let eval = minmax_alphabeta(
                            &new_board,
                            player,
                            (player + 1) % 2,
                            false,
                            0,
                            max_depth,
                            -10_000_000,
                            10_000_000,
                            &mut local_cpt,
                            &mut local_cpt_cut,
                        );
                        values.lock().unwrap()[color][hole] = eval;
                        *cpt.lock().unwrap() += local_cpt;
                        *cpt_cut.lock().unwrap() += local_cpt_cut;
                    }
                });
            }
        }).unwrap();
        max_depth += 1;
        if now.elapsed().as_secs_f32() < 0.275 {
            max_depth += 1;
        }
    }

    let mut indices = (0, 0);
    let mut max_value = values.lock().unwrap()[indices.0][indices.1];
    for color in 0..2 {
        for hole in 0..16 {
            let value = values.lock().unwrap()[color][hole];
            if value > max_value {
                indices.0 = color;
                indices.1 = hole;
                max_value = value;
            }
        }
    }

    println!("Depth : {}, for {} move(s) available.", max_depth, moves_amount);
    println!(
        "{} minmax calls, with {} alphabeta cuts.\nElapsed time : {}s",
        cpt.lock().unwrap(),
        cpt_cut.lock().unwrap(),
        now.elapsed().as_secs_f32()
    );

    return (Color::from_integer(indices.0), indices.1);
}

fn minmax_alphabeta(board: &Board, max_player: usize, player: usize, is_max: bool, depth: i32, max_depth: i32, alpha: i32, beta: i32, cpt: &mut i32, cpt_cut: &mut i32) -> i32 {
   *cpt += 1;
    let mut alpha = alpha;
    let mut beta = beta;

    if depth == max_depth || board.is_final_position() {
        return evaluation(board, max_player, depth);
    }

    let mut moves = [EvaluatedBoard::new(*board, i32::MIN); 32];
    let mut move_index = 0;
    for color in 0..2 {
        for hole in 0..16 {
            if board.is_this_move_possible(player, hole, Color::from_integer(color)) {
                moves[move_index]
                    .board
                    .play_move(player, hole, Color::from_integer(color));
                moves[move_index].eval = evaluation(&moves[move_index].board, max_player, depth);
            }
            move_index += 1;
        }
    }
    for i in 0..32 {
        let x_ev = moves[i];
        let mut j = i;
        while j > 0
            && if is_max {
                moves[j - 1].eval < x_ev.eval
            } else {
                moves[j - 1].eval > x_ev.eval
            }
        {
            moves[j] = moves[j - 1];
            j -= 1;
        }

        moves[j] = x_ev;
    }

    let mut value: i32;
    if is_max {
        value = -10_000_000;
        for i in 0..32 {
            if moves[i].eval > i32::MIN {
                let eval = minmax_alphabeta(&moves[i].board, max_player, (player + 1) % 2, false,
                    depth + 1, max_depth, alpha, beta, cpt, cpt_cut);
                value = max(value, eval);
                if value >= beta {
                    *cpt_cut += 1;
                    break;
                }
                alpha = max(alpha, value);
            }
        }
    } else {
        value = 10_000_000;
        for i in 0..32 {
            if moves[i].eval > i32::MIN {
                let eval = minmax_alphabeta(&moves[i].board, max_player, (player + 1) % 2, true,
                    depth + 1, max_depth, alpha, beta, cpt, cpt_cut);
                value = min(value, eval);
                if alpha >= value {
                    *cpt_cut += 1;
                    break;
                }
                beta = min(beta, value);
            }
        }
    }

    return value;
}

fn evaluation(board: &Board, max_player: usize, depth: i32) -> i32 {
    if board.is_winning(max_player) {
        return 10_000_000 - depth;
    }
    if board.is_loosing(max_player) {
        return -10_000_000 + depth;
    }
    if board.is_draw() {
        return 0;
    }

    let opponent = (max_player + 1) % 2;
    let diff_seed_attic = board.players_attics[max_player] - board.players_attics[opponent];

    let nb_red_seed_pair = board.red_holes[0]
        + board.red_holes[2]
        + board.red_holes[4]
        + board.red_holes[6]
        + board.red_holes[8]
        + board.red_holes[10]
        + board.red_holes[12]
        + board.red_holes[14];
    let nb_blue_seed_pair = board.blue_holes[0]
        + board.blue_holes[2]
        + board.blue_holes[4]
        + board.blue_holes[6]
        + board.blue_holes[8]
        + board.blue_holes[10]
        + board.blue_holes[12]
        + board.blue_holes[14];
    let nb_seed_pair = nb_red_seed_pair + nb_blue_seed_pair;

    let nb_red_seed_impair = board.red_holes[1]
        + board.red_holes[3]
        + board.red_holes[5]
        + board.red_holes[7]
        + board.red_holes[9]
        + board.red_holes[11]
        + board.red_holes[13]
        + board.red_holes[15];
    let nb_blue_seed_impair = board.blue_holes[1]
        + board.blue_holes[3]
        + board.blue_holes[5]
        + board.blue_holes[7]
        + board.blue_holes[9]
        + board.blue_holes[11]
        + board.blue_holes[13]
        + board.blue_holes[15];
    let nb_seed_impair = nb_red_seed_impair + nb_blue_seed_impair;

    if max_player == 0 {
        return diff_seed_attic * 3 + (64 - nb_seed_impair) + (nb_blue_seed_pair);
    } else {
        return diff_seed_attic * 3 + (64 - nb_seed_pair) + (nb_blue_seed_impair);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_minmax() {
        let mut b = Board::new();
        b.play_move(0, 0, Color::Red);
        let indice = decision_minmax(&b, 1);
        println!(
            "{} {}",
            (indice.1) + 1,
            match indice.0 {
                Color::Red => "R",
                Color::Blue => "B",
            }
        );
    }
}
