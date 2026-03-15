use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read");
    let mut input_iter = input.split_whitespace();
    let h: usize = input_iter.next().unwrap().parse().unwrap();
    let l: usize = input_iter.next().unwrap().parse().unwrap();
    let total_cans = h + l - 1;
    println!("{}", total_cans - h);
    println!("{}", total_cans - l);
}
