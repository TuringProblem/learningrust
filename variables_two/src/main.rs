#![allow(unused)]
/// above ignores unused things

fn main() {
    /// first example
    /// let x: i32 = -5;
    /// won't work x += 1;
    let mut x: i32 = 5;
    x += 1;

    let y = -123;
    /// type is inferred becasue it's negative it knows it's an i32

    const NUM: u32 = 10;

    let y: i32 = 5;
    let y: bool = true;

    let b: Vec<i32> = vec![1, 2, 3];
    /// types inferred
    let c: Vec<_> = vec![1, 2, 3];
}
