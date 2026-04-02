use std::io::{self, Write, stdout};

pub fn get_user_guess() -> Result<i32,()>{
for _ in 0..5 {
    print!("enter you guess>");
    stdout().flush().unwrap();
    let mut inpt = String::new();
    io::stdin().read_line(&mut inpt).unwrap();
    match inpt.trim().parse::<i32>(){
    Ok(num)=> return Ok(num),
    Err(_)=> {
        eprintln!("enter a number!");
    }
};
}
panic!("player is not guessing teh number correctly, hence aborting");
}