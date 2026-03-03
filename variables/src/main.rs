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
}

fn three_hours() -> u32 {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    THREE_HOURS_IN_SECONDS
}

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
