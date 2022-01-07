use crate::board::*;
use crate::minmax::*;
use std::io;

pub struct Engine {
    game_board: Board,
    actual_player: usize,
    ai_player: usize
}

impl Engine {
    pub fn new() -> Engine {
        let want_to_start = Engine::ask_to_start();
        return Engine {
            game_board: Board::new(),
            actual_player: 0,
            ai_player: want_to_start as usize
        }
    }

    pub fn run(&mut self) {
        let mut turn_number = 0;

        loop {
            turn_number += 1;
            println!("");

            println!("PLAYER {} TURN {}", self.actual_player + 1, turn_number);
            println!("{}", self.game_board);

            if self.game_board.check_famine(self.actual_player) {
                Engine::display_winner((self.actual_player + 1) % 2);
                break;
            }

            let (color, hole): (Color, usize);
            
            if self.actual_player == self.ai_player {
                let indices = decision_minmax(&self.game_board, self.actual_player);
                println!("IA PLAY {} {}", indices.1 + 1, indices.0);
                color = indices.0;
                hole = indices.1;
            }
            else {
                let (choice_color, choice_hole) = self.ask_choice();
                color = choice_color;
                hole = choice_hole;
            }
            
            self.game_board.play_move(self.actual_player, hole, color);

            let winner = self.game_board.check_win(false);
            if winner < 3 {
                println!("{}", self.game_board);
                Engine::display_winner(winner);
                break;
            }

            self.update_actual_player();
            println!("");
        }
    }

    fn display_winner(winner: usize) {
        println!("");
        if winner == 2 {
            println!("It's a draw !");
        }
        else if winner == 0 || winner == 1 {
            println!("Player {} won the game !", winner + 1);
        }
    }

    fn update_actual_player(&mut self) {
        self.actual_player = (self.actual_player + 1) % 2;
    }

    fn ask_choice(&self) -> (Color, usize) {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Couldn't read the input.");
            let vec: Vec<&str> = buffer.split_whitespace().collect();
    
            if vec.len() >= 2 {
                let hole: usize;
                let color: Color;
    
                hole = match vec[0].parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => continue
                };
                color = match vec[1].parse::<char>() {
                    Ok(c) => if c == 'R' {Color::Red} else if c == 'B' {Color::Blue} else {continue},
                    Err(_) => continue
                };
                
                if hole > 0 && self.game_board.is_this_move_possible(self.actual_player, hole - 1, color) {
                    return (color, hole - 1)
                }
                else {
                    println!("Coup invalide !");
                    continue;
                }
            }
        }
    }
    
    fn ask_to_start() -> bool {
        println!("Are you the first player ? (true/false)");
    
        loop {
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Couldn't read the input");
            let choice: bool = match choice.trim().parse() {
                Ok(v) => v,
                Err(_) => continue
            };
            return choice;
        }
    }
}