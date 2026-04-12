use proconio::input;

fn main() {
    println!("Enter x y:");

    input! {
        x: f64,
        y: f64
    }

    if x > 0.0 && y > 0.0 {
        println!("1");
    } else if x < 0.0 && y > 0.0 {
        println!("2");
    } else if x < 0.0 && y < 0.0 {
        println!("3");
    } else if x > 0.0 && y < 0.0 {
        println!("4");
    } else if x == 0.0 && y == 0.0 {
        println!("Origin");
    } else if x == 0.0 {
        println!("X-Axis");
    } else {
        println!("Y-Axis");
    }
}
