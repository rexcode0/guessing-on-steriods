

use colored::Colorize;
use guessing_on_steriods::utils::{inputs::*, player::{Player, print_table}};

fn main() {
    let mut player_list: Vec<Player>=Vec::new();
    println!("Wlecome to guess-on-steriods");
    
    let range = get_range();    
    
   for index in 1.. {
    let decision:String = get_input(format!("{}","wanna continue?[y/n] ".blink()).as_str());
    if decision.trim()!="y" {
        break;
    }
      let attempts = single_play(range.clone());
      let player = Player::new(index, attempts);
      player_list.push(player);
      }
      player_list.sort_by(|a,b| a.attempts.cmp(&b.attempts));
      print_table(&player_list);
}
