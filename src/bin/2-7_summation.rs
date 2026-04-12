use std::{env, time::Instant};

fn get_sum_loop(n: u128) -> u128 {
    let mut sum: u128 = 0;
    for i in 1..=n {
        sum += i * (n - i + 1);
    }

    sum
}

fn get_sum_formula(n: u128) -> u128 {
    (n * (n + 1) * (n + 2)) / 6
}

fn main() {
    let n: u128 = env::args().nth(1).unwrap().parse().unwrap();
    // summation n, i = 1 |і * (n - i + 1)|
    let formula_start = Instant::now();
    println!("Formula: {}", get_sum_formula(n));
    println!("Formula took {} µs", formula_start.elapsed().as_micros());
    let loop_start = Instant::now();
    println!("Loop: {}", get_sum_loop(n));
    println!("Loop took {} µs", loop_start.elapsed().as_micros());
}
