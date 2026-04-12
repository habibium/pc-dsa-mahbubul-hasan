use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    let n: isize = input.trim().parse().unwrap();
    let range = if n < 1 { n..=1 } else { 1..=n };
    println!("{}", range.sum::<isize>());
}
