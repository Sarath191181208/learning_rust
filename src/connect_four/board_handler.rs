use colored::{ColoredString, Colorize};

pub const NO_COLS: u8 = 4;
pub const NO_ROWS: u8 = 4;

pub enum BoardStates {
    Empty = 0,
    Red = 1,
    Yellow = -1,
}

pub struct Board {
    pub board: [[i8; NO_COLS as usize]; NO_ROWS as usize],
    pub is_red_playing: bool,
    pub is_game_over: bool,
    pub no_placed: u8,
}

impl Board {
    pub fn is_game_over(&self) -> bool {
        return self.is_game_over;
    }

    pub fn print_header(&self) {
        println!("### Columns:  ###");
        println!("{}-", "----".repeat(NO_COLS as usize));
        println!("{}", "| 1 | 2 | 3 | 4 |".bold().purple());
    }

    pub fn print_board(&self) {
        println!("{}-", "----".repeat(NO_COLS as usize));
        for col in self.board.iter() {
            for ele in col.iter() {
                print!("| {} ", get_ele_str_with_color(*ele).bold());
            }
            println!("|\n{}-", "----".repeat(NO_COLS as usize));
        }
    }

    pub fn place_in_col(&mut self, col_no: u8) {
        let col_no: usize = col_no.try_into().unwrap();

        if col_no > NO_COLS as usize {
            println!("{}", "Invalid column number".red());
            return;
        }

        for row in self.board.iter_mut().rev() {
            if row[col_no - 1] == BoardStates::Empty as i8 {
                row[col_no - 1] = if self.is_red_playing {
                    BoardStates::Red as i8
                } else {
                    BoardStates::Yellow as i8
                };
                break;
            }
        }

        self.no_placed += 1;
        if self.no_placed >= NO_COLS * NO_ROWS || self.check_win() {
            self.is_game_over = true;
        }
        self.is_red_playing = !self.is_red_playing;
    }

    fn check_arr_for_win(&self, arr: &[i8]) -> bool {
        let mut count = 0;
        for ele in arr.iter() {
            let is_count_for_same_color: bool = count == 0 || (*ele != 0 && count / *ele > 0);
            count = if is_count_for_same_color {
                count + *ele
            } else {
                0
            };

            if count.abs() == 4 {
                if count > 0 {
                    println!("Congratualtions, {} wins!", "Red".red().bold());
                } else {
                    println!("Congratualtions, {} wins!", "Yellow".yellow().bold());
                }
                return true;
            }
        }
        return false;
    }

    fn get_row(&self, row_idx: usize) -> &[i8] {
        let row_no: usize = row_idx.try_into().unwrap();
        return &self.board[row_no];
    }

    fn get_col(&self, col_idx: usize) -> [i8; NO_ROWS as usize] {
        let mut col_arr = [0 as i8; NO_ROWS as usize];
        for row_idx in 0..NO_ROWS as usize {
            col_arr[row_idx] = self.board[row_idx][col_idx];
        }
        return col_arr;
    }

    fn check_win(&mut self) -> bool {
        // checking in row for a win
        for row_idx in 0..NO_ROWS as usize {
            if self.check_arr_for_win(self.get_row(row_idx)) {
                return true;
            }
        }

        // checking in row col a win
        for col_idx in 0..NO_COLS as usize {
            if self.check_arr_for_win(&self.get_col(col_idx)) {
                return true;
            }
        }

        // cheking in diagonal for a win
        let mut fwd_diag_arr = [0 as i8; NO_ROWS as usize];
        let mut back_diag_arr = [0 as i8; NO_ROWS as usize];

        for row_idx in 0..NO_ROWS as usize {
            fwd_diag_arr[row_idx] = self.board[row_idx][row_idx];
            back_diag_arr[row_idx] = self.board[row_idx][NO_COLS as usize - row_idx - 1];
        }

        if self.check_arr_for_win(&fwd_diag_arr) || self.check_arr_for_win(&back_diag_arr) {
            return true;
        }

        return false;
    }
}

fn get_ele_str_with_color(ele: i8) -> ColoredString {
    if ele == BoardStates::Red as i8 {
        return "R".red();
    } else if ele == BoardStates::Yellow as i8 {
        return "Y".yellow();
    } else {
        return " ".white();
    }
}
