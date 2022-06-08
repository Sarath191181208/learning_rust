use colored::Colorize;
use std::io::{self, Write};

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn _clean_input(input: &str) -> String {
    return input
        .trim()
        .to_lowercase()
        .replace(" ", "")
        .replace("\t", "");
}

pub fn take_input(no_cols: u8) -> u8 {
    fn _print_err_msg() {
        println!("{}", "Invalid input, please try again".red());
    }

    print!("{}", "Enter column number: ".cyan());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");

    input = _clean_input(&input);
    let num: i8 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            _print_err_msg();
            return take_input(no_cols);
        }
    };

    if num < 1 || num > no_cols.try_into().unwrap() {
        _print_err_msg();
        return take_input(no_cols);
    }

    return num.try_into().unwrap();
}
