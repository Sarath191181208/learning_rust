use colored::{ColoredString, Colorize};
use rand::Rng;
use std::io::{self, Read, Write};

fn print_round(round: i32) {
    println!(
        "{} Round:{} {}",
        "###".repeat(5),
        round.to_string().purple(),
        "###".repeat(5)
    );
}

fn print_helper_text(clrd_opts: &[ColoredString]) {
    print!(
        "Please input your choice: 0) {}, 1) {}, 2) {} (or) 'q' to quit: ",
        clrd_opts[0], clrd_opts[1], clrd_opts[2]
    );
    std::io::stdout().flush().unwrap();
}

fn take_input() -> String {
    let mut choice_str = String::new();

    io::stdin()
        .read_line(&mut choice_str)
        .expect("Failed to read line");

    choice_str = choice_str.trim().to_string();
    return choice_str;
}

fn is_player_winner(player_choice: i8, computer_choice: i8) -> i8 {
    if player_choice == computer_choice {
        return 0;
    }
    let _is_player_lost = match player_choice {
        // rock vs paper
        0 => computer_choice == 1,
        // paper vs scissors
        1 => computer_choice == 2,
        // scissors vs rock
        2 => computer_choice == 0,
        _ => false,
    };
    if _is_player_lost {
        return -1;
    }
    return 1;
}

fn print_scores(player_score: i8, computer_score: i8) {
    println!("{}", "---".repeat(13));
    println!(
        "| {}: {} | {}: {} |",
        "Player Score".blue(),
        player_score,
        "Computer Score".red(),
        computer_score
    );
    println!("{}", "---".repeat(13));
}

fn print_choices(player: i8, computer: i8, clrd_opts: &[ColoredString]) {
    println!(
        "\n{} {} {}",
        clrd_opts[player as usize],
        "VS".yellow(),
        clrd_opts[computer as usize]
    );
}

pub fn play_game() {
    let options = ["Rock".red(), "Paper".blue(), "Scissors".magenta()];

    let mut comp_score = 0;
    let mut human_score = 0;
    let mut no_round = 1;

    loop {
        // clearing the screen
        print!("\x1B[2J\x1B[1;1H");

        print_round(no_round);
        print_scores(human_score, comp_score);
        print_helper_text(&options);

        let comp_choice_idx = rand::thread_rng().gen_range(1, options.len());
        // taking input from the user
        let choice_str = take_input();

        // exiting the game on 'q'
        if choice_str == "q" || choice_str == "Q" || choice_str == "quit" {
            break;
        }

        // parsing input into int
        let player_choice_idx: i8 = match choice_str.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // checking if input is valid
        if player_choice_idx < 0 || player_choice_idx > 2 {
            continue;
        }
        // checking if the game is won by the player or draw
        let winner = is_player_winner(player_choice_idx, comp_choice_idx.try_into().unwrap());

        // printing the choices taken by the computer and the player
        print_choices(
            player_choice_idx,
            comp_choice_idx.try_into().unwrap(),
            &options,
        );

        // printing the winner
        if winner == 1 {
            println!("{}", format!("You won!").bold().green());
            human_score += 1;
        } else if winner == -1 {
            println!("{}", format!("You lost!").red());
            comp_score += 1;
        } else {
            println!("{}", format!("It's a draw!"));
        }

        // press any key to continue
        print!("Press any key to continue...");
        std::io::stdout().flush().unwrap();
        // waiting for a key press
        let _ = io::stdin().read(&mut [0u8]).unwrap();
        no_round += 1;
    }
}
