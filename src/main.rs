use std::cmp::Ordering;

use guessing_on_steriods::utils::inputs::*;

fn main() {
    println!("Wlecome to guess-on-steriods");
    let system_guess: i32 = fastrand::i32(0..=90);
    let mut gu: i32;
    'outer: loop {
        gu = get_user_guess().unwrap();
        match gu.cmp(&system_guess) {
            Ordering::Less => println!("entered number is small"),
            Ordering::Equal => {
                println!("congrats you won!");
                break 'outer;
            }
            Ordering::Greater => println!("given number is greater than the guess"),
        }
    }
}



