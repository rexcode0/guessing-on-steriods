use colored::Colorize;
use guessing_on_steriods::utils::{
    inputs::*,
    player::{Player, create_leaderboard},
};
fn main() {
    dotenv::dotenv().ok();
    let mut player_list: Vec<Player> = Vec::new();
    println!("Wlecome to guess-on-steriods");
    let range = get_range();
let mut index = 1;
    loop {
        if !get_user_decision(format!("{}","want to continue".cyan()).as_str()) {
            break;
        }
        let attempts = match single_play(range.clone()){
            Ok(attempts) => attempts,
            Err(UserErrors::NoMoreAttempts)=> {
                println!("{}","i m surprised if someone can be be wrong this many times lets abort this session and start with a new one be a new player now".magenta());
                continue;
            },
            Err(UserErrors::WrongTypeOfInput)=> {
                println!("{}","why are you kindly not entering a number ? enter a numebr we have to restart this session ");
                continue;
            }
        };
        let player = Player::new(index, attempts);
        player_list.push(player);
        index+=1;
    }
    player_list.sort_by(|a, b| a.attempts.cmp(&b.attempts));
    let leaderboard = create_leaderboard(&player_list);
    write_to_file(leaderboard);
}
