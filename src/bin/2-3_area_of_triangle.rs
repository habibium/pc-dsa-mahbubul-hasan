use std::io;

fn calculate_area(sides: (f64, f64, f64)) -> f64 {
    let (a, b, c) = sides;
    let s = (a + b + c) / 2.0;

    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

fn main() {
    let mut input = String::new();

    println!("Enter the lengths of 3 sides of the triangle (S S S):");
    io::stdin().read_line(&mut input).unwrap();

    let mut sides = input.split_whitespace().map(|s| s.parse::<f64>().unwrap());

    let a = sides.next().unwrap();
    let b = sides.next().unwrap();
    let c = sides.next().unwrap();

    let area = calculate_area((a, b, c));

    println!("{:.2}", area);
}
