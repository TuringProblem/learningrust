fn main() {
    println!("{} days", 31);

    example_one();
    example_two();
    example_three();
    example_four();
}

fn example_one() {
    println!("{0}, this {1}. {1}, this is {0}\n", "Tay", "Andrew");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
}

fn example_two() {
    println!("Base 10: {}\n", 69420);
    println!("Base 2: {:b}\n", 69420);
    println!("Base 8: {:o}\n", 69420);
    println!("Base 16: {:x}\n", 69420);
}

fn example_three() {
    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:0>width$}\n", number = 1, width = 5);
}

fn example_four() {
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("Here is the output: {number:>width$} - woah, it moved 5 spaces??");
}
