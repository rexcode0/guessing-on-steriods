use crate::utils::inputs::{UserErrors, get_user_guess};
use colored::Colorize;
use std::{cmp::Ordering, env, ops::Range};
/// in this one user had to guess out of a range and will know if their guess is smaller or bigger than couputer guess
/// takes range of numbers to play within , and plays for that much time
pub fn single_play(range: Range<i32>) -> Result<i32, UserErrors> {
    let system_guess = fastrand::i32(range.clone());

    //getting the value of max_input attempts from enviromment varuable or .env file

    let max_attempts = match env::var("MAX_GUESS_ATTEMPTS") {
        Ok(attempts) => attempts.parse::<i32>().unwrap_or_else(|_| {
            eprintln!("unable to parse ,defaulting to 100");
            100
        }),
        Err(_) => {
            eprintln!(
                "{}",
                "unable to find variable MAX_GUESS_ATTEMPTS in the env, is file there? defaulting to "
            );
            100
        }
    };

    //main loop starts here

    for atempt in 1..=max_attempts {
        let user_guess: i32 = match get_user_guess() {
            Ok(num) => num,
            Err(_) => {
                return Err(UserErrors::WrongTypeOfInput);
            }
        };
        match user_guess.cmp(&system_guess) {
            Ordering::Less => println!("{}", "your entered guess is smaller".yellow()),
            Ordering::Equal => {
                println!("{}", "congrats you won!");
                return Ok(atempt);
            }
            Ordering::Greater => println!("{}", "your entered guess is bigger".cyan().italic()),
        }
    }
    Err(UserErrors::NoMoreAttempts)
}

/// here player will not be told if the given number is bigger or smaller , but only if current number is closer or farther than
/// previously guessed number that you have guessed
///
/// takes range as parameter like simgle play
pub fn single_play_hard(range: Range<i32>) -> Result<i32, UserErrors> {
    let system_guess = fastrand::i32(range.clone());

    //getting the value of max_input attempts from enviromment varuable or .env file

    let max_attempts = match env::var("MAX_GUESS_ATTEMPTS") {
        Ok(attempts) => attempts.parse::<i32>().unwrap_or_else(|_| {
            eprintln!("unable to parse ,defaulting to 100");
            100
        }),
        Err(_) => {
            eprintln!(
                "{}",
                "unable to find variable MAX_GUESS_ATTEMPTS in the env, is file there? defaulting to "
            );
            100
        }
    };

    //main loop starts here
    let mut last_guess: i32 = match get_user_guess() {
        Ok(num) => num,
        Err(_) => {
            return Err(UserErrors::WrongTypeOfInput);
        }
    };
    for atempt in 1..=max_attempts {
        let current_guess: i32 = match get_user_guess() {
            Ok(num) => num,
            Err(_) => {
                return Err(UserErrors::WrongTypeOfInput);
            }
        };
        let current_guess_distance = (current_guess - system_guess).abs();
        let last_guess_distance = (last_guess - system_guess).abs();
        if current_guess == system_guess {
            println!("{}", "congrats you won!".on_yellow().green());
            return Ok(atempt);
        }
        match current_guess_distance.cmp(&last_guess_distance) {
            Ordering::Less => println!("{}", "current guess is closer than last guess".yellow()),
            Ordering::Equal => {
                println!("{}", "teh current guess is equally farther as last guess".cyan());
            }
            Ordering::Greater => println!("{}", "current guess is farther than last guess".red()),
        }
        last_guess = current_guess;
    }
    Err(UserErrors::NoMoreAttempts)
}
