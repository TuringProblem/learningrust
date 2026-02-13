use std::io;

/**
 * Author: { @Override }
 **/

fn main() {
    println!("Guess the number!");

    println!("Please enter a number: ");

    let mut input = String::with_capacity(10);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You guessed: {input}");
}
