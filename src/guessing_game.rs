use std::io::{self, Write};
use colored::Colorize;
use rand::Rng;

fn _is_guess_same(guess: i8, secret_number: i8) -> bool {
    if guess < secret_number{
        println!("{}", format!("Too low!").blue().bold());
    }
    else if guess > secret_number {
        println!("{}", format!("Too high!").red());
    }
    return guess == secret_number;
}

pub fn play_game() {
    println!("Guess the number! press 'q' to quit");

    let rand_num = rand::thread_rng().gen_range(1, 101);
    loop {
        print!("Please input your guess: ");
        std::io::stdout().flush().unwrap();

        let mut guess_str = String::new();

        io::stdin().read_line(&mut guess_str)
            .expect("Failed to read line");

        guess_str = guess_str.trim().to_string();

        if guess_str == "q" || guess_str == "Q" || guess_str == "quit" {
            break;
        }
        
        let guess:i8 = match guess_str.parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        let seperator = "---".repeat(10);

        if _is_guess_same(guess, rand_num){
            println!("{} \n {} \n {}",seperator, format!("You gussed it Congrats 🎉!").green(), seperator);
            break;
        }
    }

}
