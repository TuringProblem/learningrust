#![warn(unused_imports)]

use std::io;

fn main() {
    println!("Welcome to the second game!\n");
    start_game();
}

fn start_game() {
    let value: Vec<u32> = handle_input();
    println!("The values are: {value:?}");
}

fn handle_input() -> Vec<u32> {
    println!("Please enter values, seperated by a comma");
    let mut input: String = String::with_capacity(50);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    parse_input(input)
}

///
/// expect input: 1, 2, 3, 4, ..., n <= 50
/// ----------------------------------
///
///
///
fn parse_input(input: String) -> Vec<u32> {
    let mut arr: Vec<u32> = Vec::new();
    for i in input.split(',') {
        arr.push(i.trim().parse().expect("Please enter a number: "));
    }
    return arr;
}
