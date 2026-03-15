use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read");
    let mut input_iter = input.split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let a: usize = input_iter.next().unwrap().parse().unwrap();
    let b: usize = input_iter.next().unwrap().parse().unwrap();
    println!("{}", 2 * n * a * b);
}
