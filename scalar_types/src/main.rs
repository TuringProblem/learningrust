#![allow(unused_variables)]
fn main() {
    int_signed_scalar();
    int_unsigned_scalar();

    let (min, max) = get_min_max32();
    let (min_char, max_char) = get_min_max_char();

    println!("i32 min: {}\ni32max: {}\n", min, max);
    println!("min char: {}\nmax char: {}\n", min_char, max_char);
    overflow_example();
}

///How to think of ints - -(2**(n-1)) -> (2**(n-1)-1)
/// so for 8 bit ints =>> -128 to 127
fn int_signed_scalar() {
    let i0: i8 = 0;
    let i1: i16 = 1;
    let i2: i32 = 2;
    let i3: i64 = 3;
    let i4: i128 = 4;
    /// depends on the size of the machine
    let i_size: isize = 1;

    println!("{}", i_size);
    println!(
        "i0: {0}\ni1: {1}\ni2: {2}\ni3: {3}\ni4: {4}\n",
        i0, i1, i2, i3, i4
    );
}

fn int_unsigned_scalar() {
    let u0: u8 = 0;
    let u1: u16 = 1;
    let u2: u32 = 2;
    let u3: u64 = 3;
    let u4: u128 = 4;
    let u_size: usize = 1;
}

fn get_min_max32() -> (i32, i32) {
    return (i32::MIN, i32::MAX);
}

fn get_min_max_char() -> (char, char) {
    return (char::MIN, char::MAX);
}

fn overflow_example() {
    let mut a: u8 = u8::MAX;
    a += 1;
    println!("{} OOOO - there is an overflow!\n", a);
}
