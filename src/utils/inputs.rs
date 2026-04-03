use colored::Colorize;
use std::{
    cmp::Ordering,
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
    if from>=to{
        println!("{}","enter a valid range ie from < to".red());
        continue;
    }
    return from..to
    }
   
}
pub fn single_play(range: Range<i32>) -> Result<i32,UserErrors> {
    let system_guess = fastrand::i32(range.clone());
    dbg!(system_guess);
    for atempt in 1..3 {
        let user_guess: i32 = match get_user_guess(){
            Ok(num)=> num,
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

pub fn get_user_decision(prompt:&str) -> bool{
     let decision:String= get_input(format!("{} [y/n]: ",prompt).as_str());
     match decision.to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false
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
