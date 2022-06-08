mod board_handler;
use board_handler::{Board, BoardStates, NO_COLS, NO_ROWS};
use colored::Colorize;
mod terminal_prompts_handler;

pub fn play_game() {
    let empty_board = [[BoardStates::Empty as i8; NO_COLS as usize]; NO_ROWS as usize];

    let mut board: Board = Board {
        board: empty_board,
        is_red_playing: true,
        is_game_over: false,
        no_placed: 0,
    };

    terminal_prompts_handler::clear_screen();

    loop {
        board.print_header();
        board.print_board();
        let col_no: u8 = terminal_prompts_handler::take_input(NO_COLS);
        board.place_in_col(col_no);
        terminal_prompts_handler::clear_screen();

        if board.is_game_over() {
            println!("### {} ###", "Game over".green().bold());
            break;
        }
    }
    board.print_board();
}
