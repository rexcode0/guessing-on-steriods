use colored::Colorize;
use std::{

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
            Err(_) => println!("{}","unable to parse the given input".red()),
        };
    }
}

pub fn write_to_file(leaderboard: &String) {
    if get_user_decision("wanna write leaderboard to file? ") {
        let mut file = match File::create("leaderboard.txt") {
            Ok(file) => file,
            Err(_) => {
                eprint!("unable to create file ");
                return;
            }
        };
        match file.write(leaderboard.as_bytes()) {
            Ok(file) => file,
            Err(_) => {
                eprint!("unable to write to file");
                return;
            }
        };
    }
}
