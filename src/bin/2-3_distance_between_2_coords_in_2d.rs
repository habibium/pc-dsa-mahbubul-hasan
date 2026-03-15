use std::io::{self, Read};

fn main() {
    println!("Enter the co-ordinates in this format (x1,y1) (x2,y2)");
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read");
    let mut coords = input
        .split(['(', ')', ',', ' ', '\n'])
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f64>().unwrap());

    let x1: f64 = coords.next().unwrap();
    let y1: f64 = coords.next().unwrap();
    let x2: f64 = coords.next().unwrap();
    let y2: f64 = coords.next().unwrap();

    println!("{}", (x2 - x1).hypot(y2 - y1));
}
