use rand::Rng;
use std::cmp::Ordering;
use std::io;

/**
 * Author: { @Override }
 **/

fn main() {
    println!("Guess the number!");

    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut is_over: bool = true;

    while is_over {
        let input: String = retrieve_input_data();

        let input: u32 = input
            .trim()
            .parse()
            .expect("Please enter a number between 1 and 100");

        match input.cmp(&random_number) {
            Ordering::Less => println!("You guessed: {input} and the number was Less than"),
            Ordering::Greater => {
                println!("You guessed: {random_number} and the secret number was greater")
            }
            Ordering::Equal => {
                println!("YOU WON!");
                is_over = false
            }
        }

        println!("Continue? y = yes, n = no");
        let mut continue_line: String = String::with_capacity(1);
        io::stdin().read_line(&mut continue_line).expect("Failed to read line");

        match continue_line.trim() {
            "y" => continue,
            "n" => {
                is_over = false; 
                println!("Have a good day :)")
            }
            _ => println!("alright, we are gonna go again then lol...")
        }
    }
}

fn retrieve_input_data() -> String {
    println!("Please enter a number between 1 and 100: ");
    let mut input: String = String::with_capacity(10);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}
