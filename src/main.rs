use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn reset() -> bool {
    let mut play_again: String = String::new();
    println!("Would you like to play again?, yes (Y), any other key to finish the game");
    io::stdin()
        .read_line(&mut play_again)
        .expect("Failed to read line");
    return play_again.to_lowercase().trim() == "y".to_string();
}

fn set_secret_number(minimum: u8, maximun: u8) -> u8 {
    return rand::thread_rng().gen_range(minimum, maximun);
}

fn main() {
    let minimum: u8 = 1;
    let maximun: u8 = 50;
    let mut secret_number: u8 = set_secret_number(minimum, maximun);
    let mut limit_games: u8 = 7;

    println!("Guess the number");

    loop {
        println!(
            "Please input youre guess between {} and {}, you have {} chances",
            minimum, maximun, limit_games
        );

        let mut guess: String = String::new();
        let mut finish: bool = false;

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please insert a number".red());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You win!!".green());
                finish = true;
            }
            Ordering::Greater => println!("{}", "To big".red()),
            Ordering::Less => println!("{}", "To short".red()),
        }

        limit_games -= 1;

        if limit_games == 0 {
            println!(
                "{}, the number was {}",
                "You lose, try again".red(),
                secret_number
            );
            if reset() {
                secret_number = set_secret_number(minimum, maximun);
                limit_games = 7;
            } else {
                break;
            }
        }

        if finish {
            if reset() {
                secret_number = set_secret_number(minimum, maximun);
                limit_games = 7;
            } else {
                break;
            }
        }
    }
}
