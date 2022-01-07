use std::fmt;

pub struct Board {
    pub red_holes: [i32; 16],
    pub blue_holes: [i32; 16],
    pub players_attics: [i32; 2]
}

pub enum Color {
    Red,
    Blue
}

impl Copy for Board { }

impl Clone for Board {
    fn clone(&self) -> Board {
        *self
    }
}

impl Color {
    pub fn from_integer(id: usize) -> Color {
        match id {
            0 => Color::Red,
            1 => Color::Blue,
            _ => Color::Red
        }
    }
}

impl Copy for Color { }

impl Clone for Color {
    fn clone(&self) -> Color {
        *self
    }
}


impl Board {
    pub fn new() -> Board {
        Board {
            red_holes: [2; 16],
            blue_holes: [2; 16],
            players_attics: [0; 2]
        }
    }

    pub fn get_player_attic(&self, player: usize) -> i32 {
        return self.players_attics[player];
    }

    fn distribute_red_seeds(&mut self, hole: usize) -> usize {
        let mut nb_seeds = self.red_holes[hole];
        self.red_holes[hole] = 0;

        let mut current_hole = (hole + 1) % 16;

        if nb_seeds > 0 {
            self.red_holes[current_hole] += 1;
            nb_seeds -= 1;
        }

        while nb_seeds > 0 {
            if current_hole != hole {
                current_hole = (current_hole + 1) % 16;
                self.red_holes[current_hole] += 1;
                nb_seeds -= 1;
            }
        }

        return current_hole;
    }

    fn distribute_blue_seeds(&mut self, hole: usize) -> usize {
        let mut nb_seeds = self.blue_holes[hole];
        self.blue_holes[hole] = 0;

        let mut current_hole = (hole + 1) % 16;
        if nb_seeds > 0 {
            self.blue_holes[current_hole] += 1;
            nb_seeds -= 1;
        }

        while nb_seeds > 0 {
            if current_hole != hole {
                current_hole = (current_hole + 2) % 16;
                self.blue_holes[current_hole] += 1;
                nb_seeds -= 1;
            }
        }

        return current_hole;
    }

    fn pick_seed(&mut self, last_hole: usize, player: usize) -> i32 {
        let mut current_hole = last_hole;
        let mut nb_blue_seed = self.blue_holes[current_hole];
        let mut nb_red_seed = self.red_holes[current_hole];

        let mut nb_total_seed = 0;
        while (nb_red_seed + nb_blue_seed) == 2 || nb_red_seed + nb_blue_seed == 3 {
            self.red_holes[current_hole] = 0;
            self.blue_holes[current_hole] = 0;

            nb_total_seed += nb_red_seed + nb_blue_seed;

            current_hole = current_hole.wrapping_sub(1) % 16;
            nb_blue_seed = self.blue_holes[current_hole];
            nb_red_seed = self.red_holes[current_hole];
        }

        self.players_attics[player] += nb_total_seed;
        return nb_total_seed;
    }

    pub fn play_move(&mut self, player: usize, hole: usize, color: Color) {
        let last_hole = match color {
            Color::Red => self.distribute_red_seeds(hole),
            Color::Blue => self.distribute_blue_seeds(hole)
        };
        self.pick_seed(last_hole, player);
    }

    pub fn is_this_move_possible(&self, player: usize, hole: usize, color: Color) -> bool {
        if hole >= 16 {
            return false;
        }

        if hole % 2 == player {
            match color {
                Color::Red => {
                    return self.red_holes[hole] > 0;
                }
                Color::Blue => {
                    return self.blue_holes[hole] > 0;
                }
            }
        }

        else {
            return false;
        }
    }
}

/* WIN CONDITIONS */
impl Board {
    pub fn is_final_position(&self) -> bool {
        if self.check_less_eight_seeds() {
            return true;
        }
        if self.check_has_more_than_half_seeds(0) || self.check_has_more_than_half_seeds(1) {
            return true;
        }
        if self.check_famine(0) || self.check_famine(1) {
            return true;
        }
        return false;
    }

    pub fn check_win(&self, with_famine: bool) -> usize {
        for player in 0..2 {
            if self.check_famine((player + 1) % 2) && with_famine {return player; }
            if self.check_has_more_than_half_seeds(player) { return player; }
        }

        if self.check_less_eight_seeds() {
            if self.players_attics[0] == self.players_attics[1] { return 2; }
            else { return if self.players_attics[0] > self.players_attics[1] { 0 } else { 1 }; }
        }

        return 3;
    }

    pub fn check_famine(&self, player: usize) -> bool {
        let mut i = player;
        while i < 16 {
            if self.blue_holes[i] + self.red_holes[i] != 0 {
                return false;
            }
            i += 2;
        }

        return true;
    }

    fn check_has_more_than_half_seeds(&self, player: usize) -> bool {
        return self.players_attics[player] >= 33;
    }

    fn check_less_eight_seeds(&self) -> bool {
        let mut nb_total_seed = 0;
        for i in 0..16 {
            nb_total_seed += self.red_holes[i] + self.blue_holes[i];
        }

        return nb_total_seed < 8;
    }

    pub fn is_winning(&self, player: usize) -> bool {
        return self.check_win(true) == player;
    }

    pub fn is_loosing(&self, player: usize) -> bool {
        return self.check_win(true) == (player + 1) % 2;
    }

    pub fn is_draw(&self) -> bool {
        return self.check_win(true) == 2;
    }
}






impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ss = String::new();
        
        ss.push_str        ("   1        2        3        4        5        6        7        8\n");
        ss.push_str(format!("[{}R;{}B]  [{}R;{}B]  [{}R;{}B]  [{}R;{}B]  [{}R;{}B]  [{}R;{}B]  [{}R;{}B]  [{}R;{}B]\n\n", 
                    self.red_holes[0], self.blue_holes[0],
                    self.red_holes[1], self.blue_holes[1],
                    self.red_holes[2], self.blue_holes[2],
                    self.red_holes[3], self.blue_holes[3],
                    self.red_holes[4], self.blue_holes[4],
                    self.red_holes[5], self.blue_holes[5],
                    self.red_holes[6], self.blue_holes[6],
                    self.red_holes[7], self.blue_holes[7],).as_str());

        ss.push_str(        "  16        15       14       13       12       11       10       9\n");
        ss.push_str(format!("[{}R;{}B]  [{}R;{}B]  [{}R;{}B]  [{}R;{}B]  [{}R;{}B]  [{}R;{}B]  [{}R;{}B]  [{}R;{}B]\n", 
                    self.red_holes[15], self.blue_holes[15],
                    self.red_holes[14], self.blue_holes[14],
                    self.red_holes[13], self.blue_holes[13],
                    self.red_holes[12], self.blue_holes[12],
                    self.red_holes[11], self.blue_holes[11],
                    self.red_holes[10], self.blue_holes[10],
                    self.red_holes[9],  self.blue_holes[9],
                    self.red_holes[8],  self.blue_holes[8]).as_str());

        ss.push_str(format!("\nJ1 : {}\nJ2 : {}\n", self.players_attics[0], self.players_attics[1]).as_str());
        return write!(f, "{}", ss.as_str());
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "R"),
            Color::Blue => write!(f, "B")
        }
    } 
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_init() {
        let b = Board::new();
        for i in 0..16 {
            assert_eq!(2, b.red_holes[i]);
            assert_eq!(2, b.red_holes[i]);
        }
        assert_eq!(0, b.players_attics[0]);
        assert_eq!(0, b.players_attics[1]);
    }

    #[test]
    fn test_copy() {
        let mut b1 = Board::new();
        b1.play_move(1, 2, Color::Red);
        b1.players_attics[0] = 10;

        let mut b2 = *(&mut b1);
        b2.play_move(0, 3, Color::Red);

        assert_eq!(0, b1.red_holes[2]);
        assert_eq!(0, b2.red_holes[2]);
        assert_eq!(3, b1.red_holes[3]);
        assert_eq!(0, b2.red_holes[3]);

        assert_eq!(b1.players_attics[0], b2.players_attics[0]);
        assert_eq!(b1.players_attics[1], b2.players_attics[1]);
    }

    #[test]
    fn test_distribute_red_seeds() {
        let mut b = Board::new();
        let result_five = b.distribute_red_seeds(5);
        let result_fourteen = b.distribute_red_seeds(14);

        assert_eq!(0, b.red_holes[5]);
        assert_eq!(0, b.red_holes[14]);

        assert_eq!(7, result_five);
        assert_eq!(0, result_fourteen);

        assert_eq!(3, b.red_holes[6]);
        assert_eq!(3, b.red_holes[7]);
        assert_eq!(3, b.red_holes[15]);
        assert_eq!(3, b.red_holes[0]);
        for i in 0..16 {
            if i != 5 && i != 14 && i != 6 && i != 7 && i != 15 && i != 0 {
                assert_eq!(2, b.red_holes[i]);
            }
        }

        let result_zero = b.distribute_red_seeds(0);
        assert_eq!(3, b.red_holes[1]);
        assert_eq!(3, b.red_holes[2]);
        assert_eq!(3, b.red_holes[3]);
        assert_eq!(0, b.red_holes[0]);
        assert_eq!(3, result_zero);
    }

    #[test]
    fn test_distribute_blue_seeds() {
        let mut b = Board::new();
        let result_five = b.distribute_blue_seeds(5);
        let result_fourteen = b.distribute_blue_seeds(14);

        assert_eq!(0, b.blue_holes[5]);
        assert_eq!(0, b.blue_holes[14]);

        assert_eq!(8, result_five);
        assert_eq!(1, result_fourteen);

        assert_eq!(3, b.blue_holes[6]);
        assert_eq!(3, b.blue_holes[8]);
        assert_eq!(3, b.blue_holes[15]);
        assert_eq!(3, b.blue_holes[1]);
        for i in 0..16 {
            if i != 5 && i != 14 && i != 6 && i != 8 && i != 15 && i != 1 {
                assert_eq!(2, b.blue_holes[i]);
            }
        }

        let result_zero = b.distribute_blue_seeds(1);
        assert_eq!(3, b.blue_holes[2]);
        assert_eq!(3, b.blue_holes[4]);
        assert_eq!(4, b.blue_holes[6]);
        assert_eq!(0, b.blue_holes[1]);
        assert_eq!(6, result_zero);

        assert_eq!(2, b.blue_holes[3]);
        assert_eq!(0, b.blue_holes[5]);
        assert_eq!(2, b.blue_holes[7]);
    }

    #[test]
    fn test_pick_seed() {
        let mut b = Board::new();

        b.distribute_blue_seeds(0);
        assert_eq!(0, b.blue_holes[0]);
        assert_eq!(2, b.red_holes[0]);
        assert_eq!(2, b.red_holes[1]);
        assert_eq!(3, b.blue_holes[1]);
        assert_eq!(2, b.red_holes[2]);
        assert_eq!(2, b.blue_holes[2]);
        assert_eq!(2, b.red_holes[3]);
        assert_eq!(3, b.blue_holes[3]);
        for i in 4..16 {
            assert_eq!(2, b.blue_holes[i]);
            assert_eq!(2, b.red_holes[i]);
        }

        let final_hole = b.distribute_blue_seeds(13);
        assert_eq!(0, b.blue_holes[13]);
        assert_eq!(2, b.red_holes[13]);
        assert_eq!(3, b.blue_holes[14]);
        assert_eq!(2, b.red_holes[14]);
        assert_eq!(2, b.blue_holes[15]);
        assert_eq!(2, b.red_holes[15]);
        assert_eq!(1, b.blue_holes[0]);
        assert_eq!(2, b.red_holes[0]);
        assert_eq!(0, final_hole);
        let pick_return = b.pick_seed(0, 1);
        assert_eq!(0, b.blue_holes[0]);
        assert_eq!(0, b.red_holes[0]);
        assert_eq!(3, pick_return);
        assert_eq!(3, b.players_attics[1]);

        b = Board::new();
        b.blue_holes[0] = 0;
        b.blue_holes[1] = 0;
        b.blue_holes[2] = 0;
        b.blue_holes[3] = 0;
        b.red_holes[14] = 5;
        b.blue_holes[15] = 0;
        b.distribute_red_seeds(14);

        assert_eq!(3, b.red_holes[15]);
        for i in 0..4 {
            assert_eq!(3, b.red_holes[i]);
        }

        let pick_return = b.pick_seed(3, 0);
        assert_eq!(17, pick_return);

        for i in 0..4 {
            assert_eq!(0, b.red_holes[i]);
            assert_eq!(0, b.blue_holes[i]);
        }
        for i in 14..16 {
            assert_eq!(0, b.red_holes[i]);
            assert_eq!(0, b.blue_holes[i]);
        }

        assert_eq!(0, b.players_attics[1]);
        assert_eq!(17, b.players_attics[0]);

        for i in 5..14 {
            assert_eq!(2, b.red_holes[i]);
            assert_eq!(2, b.blue_holes[i]);
        }
    }

    #[test]
    fn test_play_move() {
        let mut b = Board::new();
        b.play_move(0, 0, Color::Red);
        assert_eq!(0, b.red_holes[0]);
        assert_eq!(2, b.blue_holes[0]);

        for i in 1..3 {
            assert_eq!(3, b.red_holes[i]);
            assert_eq!(2, b.blue_holes[i]);
        }

        b.play_move(1, 13, Color::Blue);
        assert_eq!(0, b.blue_holes[13]);
        assert_eq!(2, b.red_holes[13]);
        assert_eq!(3, b.blue_holes[14]);
        assert_eq!(3, b.blue_holes[14]);
        assert_eq!(2, b.blue_holes[15]);
        assert_eq!(2, b.red_holes[15]);
        assert_eq!(0, b.blue_holes[0]);
        assert_eq!(0, b.red_holes[0]);

        assert_eq!(3, b.players_attics[1]);
        
    }

    #[test]
    fn test_is_this_move_possible() {
        let mut b = Board::new();

        assert_eq!(true, b.is_this_move_possible(0, 0, Color::Red));
        assert_eq!(true, b.is_this_move_possible(0, 0, Color::Blue));
        assert_eq!(false, b.is_this_move_possible(1, 0, Color::Red));
        assert_eq!(false, b.is_this_move_possible(1, 0, Color::Blue));

        b.play_move(0, 0, Color::Red);

        assert_eq!(false, b.is_this_move_possible(0, 0, Color::Red));
        assert_eq!(true, b.is_this_move_possible(0, 0, Color::Blue));
        assert_eq!(false, b.is_this_move_possible(1, 0, Color::Red));
        assert_eq!(false, b.is_this_move_possible(1, 0, Color::Blue));

        assert_eq!(true, b.is_this_move_possible(1, 1, Color::Red));
        assert_eq!(true, b.is_this_move_possible(1, 1, Color::Blue));

        b.play_move(1, 1, Color::Blue);

        assert_eq!(true, b.is_this_move_possible(1, 1, Color::Red));
        assert_eq!(false, b.is_this_move_possible(1, 1, Color::Blue));
        assert_eq!(false, b.is_this_move_possible(0, 1, Color::Red));
        assert_eq!(false, b.is_this_move_possible(0, 1, Color::Blue));
    }

    #[test]
    fn test_check_famine() {
        let mut b = Board::new();

        for i in 0..16 {
            if i%2 == 0 {
                b.blue_holes[i] = 0;
                b.red_holes[i] = 0;
            }
        }

        assert_eq!(true, b.check_famine(0));
        assert_eq!(false, b.check_famine(1));
        b.red_holes[14] = 1;
        assert_eq!(false, b.check_famine(0));
    }

    #[test]
    fn test_check_win() {
        let mut b = Board::new();
        for i in 0..16 {
            if i%2 == 0 {
                b.blue_holes[i] = 0;
                b.red_holes[i] = 0;
            }
        }
        assert_eq!(1, b.check_win(true));
        assert_eq!(3, b.check_win(false));

        b = Board::new();
        b.players_attics[0] = 33;
        assert_eq!(0, b.check_win(true));

        b = Board::new();
        for i in 0..14 {
            b.red_holes[i] = 0;
            b.blue_holes[i] = 0;
        }
        b.red_holes[15] -= 1;
        assert_eq!(2, b.check_win(true));

        b.red_holes[15] = 0;
        b.blue_holes[15] = 0;
        assert_eq!(0, b.check_win(true));
    }

    #[test]
    fn test_final_position() {
        let mut b = Board::new();

        for i in 0..16 {
            if i%2 == 0 {
                b.blue_holes[i] = 0;
                b.red_holes[i] = 0;
            }
        }
        assert_eq!(true, b.is_final_position());

        b = Board::new();
        b.players_attics[0] = 33;
        assert_eq!(true, b.is_final_position());

        b = Board::new();
        for i in 0..14 {
            b.red_holes[i] = 0;
            b.blue_holes[i] = 0;
        }
        assert_eq!(false, b.is_final_position());
        b.red_holes[15] -= 1;
        assert_eq!(true, b.is_final_position());

        b = Board::new();
        assert_eq!(false, b.is_final_position());
    }
}