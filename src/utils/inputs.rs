use colored::Colorize;
use std::{
    cmp::Ordering,
    env,
    fs::File,
    io::{self, Write},
    ops::Range,
    str::FromStr,
};

pub enum UserErrors {
    NoMoreAttempts,
    WrongTypeOfInput,
}
pub fn get_user_guess() -> Result<i32, UserErrors> {
    for _ in 0..5 {
        let inpt: String = get_input(format!("{}", "enter you guess>".blue()).as_str());
        match inpt.trim().parse::<i32>() {
            Ok(num) => return Ok(num),
            Err(_) => {
                eprintln!("{}", "enter a number!".red());
            }
        };
    }
    Err(UserErrors::WrongTypeOfInput)
}
pub fn get_range() -> Range<i32> {
    loop {
        println!("enter a number range");
        let from: i32 = get_input("from: ");
        let to: i32 = get_input("to: ");
        if from >= to {
            eprintln!("{}", "enter a valid range ie from < to".red());
            continue;
        }
        return from..to;
    }
}
pub fn single_play(range: Range<i32>) -> Result<i32, UserErrors> {
    let system_guess = fastrand::i32(range.clone());
    dbg!(system_guess);

    let max_attempts = match env::var("MAX_GUESS_ATTEMPTS") {
        Ok(attempts) => {
            match attempts.parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("{}","unable to parse the the number in environment variable continuing with 100".red());
                    100
                }
            }
        }
        Err(_) => {
            eprintln!(
                "{}",
                "unable to find variable MAX_GUESS_ATTEMPTS in the env, is file there? defaulting to "
            );
            100
        }
    };
    dbg!(max_attempts);
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

pub fn get_user_decision(prompt: &str) -> bool {
    let decision: String = get_input(format!("{} [y/n]: ", prompt).as_str());
    match decision.to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}
pub fn get_input<T>(prompt: &str) -> T
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(data) => return data,
            Err(_) => println!("unable to parse the given input"),
        };
    }
}

pub fn write_to_file(leaderboard: String) {
    if get_user_decision("wanna write leaderboard to file? ") {
        let mut file = match File::create("leaderboard.txt") {
            Ok(file) => file,
            Err(_) => {
                eprint!("unable to create file ");
                return;
            }
        };
        match file.write(leaderboard.as_bytes()){
            Ok(file) => file,
            Err(e) => {
                eprint!("unable to write to file");
                return;
            }
        };
    }
}
