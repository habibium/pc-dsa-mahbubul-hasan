use std::{f64::consts::PI, io};

fn main() {
    let mut input = String::new();

    println!("Enter the radius of the circle:");
    io::stdin().read_line(&mut input).unwrap();

    let r: f64 = input.trim().parse().unwrap();

    println!("{}", 2.0 * PI * r);
}
