use colored::{ColoredString, Colorize};
use std::io;
use std::io::Read;
use std::io::Write;

mod wordle_word_list;

fn print_rules() {
    println!(
        "
{}
{}
You will be given a word and you will have to guess it.
Here are the rules:
1. You have to guess the Wordle in six goes or less.
2. Every word you enter must be in the word list. ...
3. A correct letter turns {}.
4. A correct letter in the wrong place turns {}.
5. An incorrect letter turns gray.
6. Letters can be used more than once.
7. Answers are never plurals.
    ",
        "Welcome to Wordle!".purple().bold(),
        "------------------",
        "green".green().bold(),
        "yellow".yellow().bold(),
    );
    wait_for_key_press();
}

fn formmat_word(word: &str) -> String {
    return word
        .trim()
        .to_string()
        .replace(" ", "")
        .replace("\t", "")
        .to_lowercase();
}

fn get_input_word() -> String {
    print!("{}", "Please enter a word: ".blue());
    io::stdout().flush().unwrap();

    let mut input_word = String::new();
    io::stdin()
        .read_line(&mut input_word)
        .expect("Failed to read line");

    input_word = formmat_word(&input_word);

    if input_word.len() < 5 {
        println!("{}", "Please enter a valid word".red());
        return get_input_word();
    }

    if !wordle_word_list::is_in_list(&input_word) {
        println!("{}", "Isn't in the valid word list".red());
        return get_input_word();
    }

    return input_word;
}

fn are_words_same(rand_word: &str, input_word: &str) -> bool {
    let word1_chars: Vec<char> = rand_word.chars().collect();
    let word2_chars: Vec<char> = input_word.chars().collect();

    let mut match_count = 0;

    for (i, char_input) in word2_chars.iter().enumerate() {
        let mut clrd_txt: ColoredString = char_input.to_string().white();
        for (j, char_rand) in word1_chars.iter().enumerate() {
            if char_input == char_rand {
                if i == j {
                    match_count += 1;
                    clrd_txt = char_input.to_string().bold().green();
                    break;
                } else {
                    clrd_txt = char_input.to_string().bold().yellow();
                }
            }
        }
        print!("{} ", clrd_txt);
    }
    println!("");

    if match_count == rand_word.len() {
        return true;
    }
    return false;
}

fn wait_for_key_press() {
    // press any key to continue
    print!("Press enter key to continue...");
    std::io::stdout().flush().unwrap();
    // waiting for a key press
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

pub fn play_game() {
    const NO_TRAILS: i32 = 5;

    let mut won = false;
    // let mut no_won: i32 = 0;
    print_rules();
    loop {
        // clearing the screen
        print!("\x1B[2J\x1B[1;1H");

        let rand_word = wordle_word_list::get_random_word();
        let mut i = 0;
        while i < NO_TRAILS {
            println!("Chance: {}", (i + 1).to_string().purple());
            let input_word = get_input_word();
            if are_words_same(&rand_word, &input_word) {
                // no_won += 1;
                won = true;
                break;
            }
            i += 1;
        }
        if won {
            println!("{}", "You won!".green());
            won = false;
        } else {
            println!("{} the word was: {}", "You lost!".red(), rand_word.purple());
        }

        wait_for_key_press();
    }
}
