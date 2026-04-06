use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter a number");
    io::stdin().read_line(&mut input).unwrap();
    let n: f64 = input.trim().parse().unwrap();
    println!("{}", n.sqrt().round());
}
