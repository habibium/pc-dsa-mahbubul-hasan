use std::env;

fn main() {
    let mut args: Vec<i32> = env::args()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    args.sort();

    args.iter().for_each(|x| print!("{x} "));
    println!();
}
