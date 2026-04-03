use tabled::{Table, Tabled, settings::Style};

use crate::utils::inputs::get_input;
// use std::fmt::Display;
#[derive(Debug,Tabled)]
pub struct Player {
    pub player_id: usize,
    pub name: String,
    pub attempts: i32,
}
impl Player {
    pub fn new(id: usize, attempts: i32) -> Player {
        let name: String = get_input(format!("player {} name:", id).as_str());
        let player: Player = Player {
            player_id: id,
            name: name,
            attempts: attempts,
        };
        player
    }
}

pub fn print_table(table :&Vec<Player>)
{

    print!("{}",Table::new(table).with(Style::re_structured_text()));
}