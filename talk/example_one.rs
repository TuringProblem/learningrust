fn mutate_value(value: &mut i32, how_much: &i32) {
    *value += how_much;
}

fn main() {
    let mut my_value = 42;
    let how_much = 10;

    mutate_value(&mut my_value, &how_much);
    println!("Hello, World: {}", my_value);
}
