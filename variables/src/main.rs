fn main() {
    let mut x: u32 = 5;
    println!("x: {}\n", x);
    x = 6;
    println!("x: {}\n", x);
    three_hours();
    shadowing_example();
    example();
    example_two();
    example_three();
    example_four();
    example_five();
}

fn three_hours() -> u32 {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    THREE_HOURS_IN_SECONDS
}

/// This is suppose to teach me about shadowing - to me just looks like scoping... lol but I think
/// I get it, because you tech need to use mut when changing the variable, I'm pretty sure if I
/// understand, it creates a new variable, with the same name but is different lol
fn shadowing_example() {
    let x = 3;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of the outer scope x is {x}");
}

fn example() {
    let size = "     ";
    let spaces = size.len();

    println!("function 1: The length of the spaces is {spaces}");
}

fn example_two() {
    let spaces = "     ";
    let spaces = spaces.len();

    println!("function 2: The length of the spaces is {spaces}");
}

fn example_three() {
    let mut spaces = "     ";
    let spaces = spaces.len();

    println!("function 3: The length of the spaces is {spaces}");
}

fn example_four() {
    let lang = "Rust";

    println!("Thanks for coding is {lang}");
    let x = 5;

    println!("{0} * {0} = {1}", x, x * x);
}

#[derive(Debug)]
struct Language {
    language: String,
    version: String,
}

fn example_five() {
    let lang = Language {
        language: "Rust".to_string(),
        version: "1.0.0".to_string(),
    };

    /// pretty much the same thing
    println!("{lang:?}");
    println!("{:?}", lang);
    /// pretty printing version, with line breaks
    println!("{:#?}", lang);
}
