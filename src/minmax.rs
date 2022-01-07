use crate::board::*;
use std::cmp::max;
use std::cmp::min;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

struct EvaluatedBoard {
    board: Board,
    eval: i32
}

impl EvaluatedBoard {
    fn new(board: Board, eval: i32) -> EvaluatedBoard {
        EvaluatedBoard {
            board,
            eval
        }
    }
}

impl Copy for EvaluatedBoard { }

impl Clone for EvaluatedBoard {
    fn clone(&self) -> EvaluatedBoard {
        *self
    }
}

pub fn decision_minmax(board: &Board, player: usize) -> (Color, usize) {
    let mut cpt = 0;
    let mut cpt_cut = 0;
    let mut values = [[0; 16]; 2];

    let now = Instant::now();
    let max_depth = 9;
    let mut moves_amount = 0;
    for color in 0..2 {
        for hole in 0..16 {
            if board.is_this_move_possible(player, hole, Color::from_integer(color)) {
                moves_amount += 1;

                let mut new_board = *board;
                new_board.play_move(player, hole, Color::from_integer(color));
                values[color][hole] = minmax_alphabeta(&new_board, player, (player+1)%2, false, 0, max_depth, -10_000_000, 10_000_000, &mut cpt, &mut cpt_cut);
            }
            else {
                values[color][hole] = i32::MIN;
            } 
        }
    }

    let mut indices = (0, 0);
    for color in 0..2 {
        for hole in 0..16 {
            println!("{} ", values[color][hole]);
            if values[color][hole] > values[indices.0][indices.1] {
                indices.0 = color;
                indices.1 = hole;
            }
        }
        println!("");
    }

    println!("Depth : {}, for {} move(s) available.", max_depth + 1, moves_amount);
    println!("{} minmax calls, with {} alphabeta cuts.\nElapsed time : {}s", cpt, cpt_cut, now.elapsed().as_secs_f32());

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
                moves[move_index].board.play_move(player, hole, Color::from_integer(color));
                moves[move_index].eval = evaluation(&moves[move_index].board, max_player, depth);
            }
            move_index += 1;
        }
    }
    for i in 0..32 {
        let x_ev = moves[i];
        let mut j = i;
        while j > 0 && if is_max { moves[j-1].eval < x_ev.eval } else { moves[j-1].eval > x_ev.eval } {
            moves[j] = moves[j-1];
            j -= 1;
        }

        moves[j] = x_ev;
    }

    let mut value: i32;
    if is_max {
        value = -10_000_000;
        for i in 0..32 {
            if moves[i].eval > i32::MIN {
                let eval = minmax_alphabeta(&moves[i].board, max_player, (player+1)%2, false, depth + 1, max_depth, alpha, beta, cpt, cpt_cut);
                value = max(value, eval);
                if value >= beta {
                    *cpt_cut += 1;
                    break;
                }
                alpha = max(alpha, value);
            }
        }
    }
    else {
        value = 10_000_000;
        for i in 0..32 {
            if moves[i].eval > i32::MIN {
                let eval = minmax_alphabeta(&moves[i].board, max_player, (player+1)%2, true, depth + 1, max_depth, alpha, beta, cpt, cpt_cut);
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

    let nb_red_seed_pair = board.red_holes[0] +
                            board.red_holes[2] + 
                            board.red_holes[4] + 
                            board.red_holes[6] + 
                            board.red_holes[8] + 
                            board.red_holes[10] + 
                            board.red_holes[12] + 
                            board.red_holes[14];
    let nb_blue_seed_pair = board.blue_holes[0] +
                            board.blue_holes[2] + 
                            board.blue_holes[4] + 
                            board.blue_holes[6] + 
                            board.blue_holes[8] + 
                            board.blue_holes[10] + 
                            board.blue_holes[12] + 
                            board.blue_holes[14];
    let nb_seed_pair = nb_red_seed_pair + nb_blue_seed_pair;

    let nb_red_seed_impair = board.red_holes[1] +
                            board.red_holes[3] + 
                            board.red_holes[5] + 
                            board.red_holes[7] + 
                            board.red_holes[9] + 
                            board.red_holes[11] + 
                            board.red_holes[13] + 
                            board.red_holes[15];
    let nb_blue_seed_impair = board.blue_holes[1] +
                            board.blue_holes[3] + 
                            board.blue_holes[5] + 
                            board.blue_holes[7] + 
                            board.blue_holes[9] + 
                            board.blue_holes[11] + 
                            board.blue_holes[13] + 
                            board.blue_holes[15];
    let nb_seed_impair = nb_red_seed_impair + nb_blue_seed_impair;

    if max_player == 0 {
        return diff_seed_attic * 3 + (64 - nb_seed_impair) + (nb_blue_seed_pair);
    } 
    else {
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
        println!("{} {}", (indice.1) + 1, match indice.0 { Color::Red => "R", Color::Blue => "B"});
    }
}