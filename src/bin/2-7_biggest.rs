use std::env;

fn main() {
    let args = env::args().skip(1);
    let max = args
        .map(|n| n.parse::<i32>().unwrap())
        .max_by(|x, y| x.cmp(y))
        .unwrap();
    println!("{max}");
}
