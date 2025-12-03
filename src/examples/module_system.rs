mod cook_food;
use cook_food::chinese::{cook_noodles, cook_spring_rolls};
use cook_food::indian::cook_samosa;
use cook_food::italian::{cook_pasta, cook_pizza};
use cook_food::mexican;

fn main() {
    println!("From main…");
    cook_samosa();
    cook_noodles();
    cook_pizza();
    cook_pasta();
    cook_spring_rolls();
    mexican::cook_burritos();
    mexican::cook_tomato_salsa();
}
