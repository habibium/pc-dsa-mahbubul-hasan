use std::io;

fn calculate_angles(sides: (f64, f64, f64)) -> (f64, f64, f64) {
    let (a, b, c) = sides;
    // Law of cosines rearranged to calculate angles
    // A = arccos((b^2 + c^2 - a^2) / 2bc)
    let angle_a = ((b * b + c * c - a * a) / (2.0 * b * c)).acos();
    // B = arccos((a^2 + c^2 - b^2) / 2ac)
    let angle_b = ((a * a + c * c - b * b) / (2.0 * a * c)).acos();
    // C = arccos((a^2 + b^2 - c^2) / 2ab)
    let angle_c = ((a * a + b * b - c * c) / (2.0 * a * b)).acos();

    (
        angle_a.to_degrees(),
        angle_b.to_degrees(),
        angle_c.to_degrees(),
    )
}

fn main() {
    let mut input = String::new();

    println!("Enter 3 sides of the triangle (S S S)");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let mut sides = input.split_whitespace().map(|s| s.parse::<f64>().unwrap());

    let a = sides.next().unwrap();
    let b = sides.next().unwrap();
    let c = sides.next().unwrap();

    let (a_a, a_b, a_c) = calculate_angles((a, b, c));

    println!("{:.2}° {:.2}° {:.2}°", a_a, a_b, a_c);
}

#[cfg(test)]
mod tests {
    use super::*;

    // A helper macro to quickly define our 10 tests
    macro_rules! triangle_tests {
        ($($name:ident: $input:expr => $expected:expr,)*) => {
        $(
            #[test]
            fn $name() {
                // 1. Parse the input string into a tuple of 3 f64s
                let inputs: Vec<f64> = $input.split_whitespace()
                                             .map(|s| s.parse().unwrap())
                                             .collect();
                let sides = (inputs[0], inputs[1], inputs[2]);

                // 2. Call the function with the new signature
                let (angle_a, angle_b, angle_c) = calculate_angles(sides);

                // 3. Format the result to 2 decimal places to handle float precision and match "A A A"
                let result_str = format!("{:.2} {:.2} {:.2}", angle_a, angle_b, angle_c);

                // 4. Assert against the expected string
                assert_eq!(result_str, $expected);
            }
        )*
        }
    }

    // The 10 test cases remain exactly the same
    triangle_tests! {
        test_01_equilateral: "1 1 1" => "60.00 60.00 60.00",
        test_02_right: "3 4 5" => "36.87 53.13 90.00",
        test_03_isosceles: "5 5 8" => "36.87 36.87 106.26",
        test_04_scalene: "7 8 9" => "48.19 58.41 73.40",
        test_05_right_alt: "5 12 13" => "22.62 67.38 90.00",
        test_06_large_int: "100 100 100" => "60.00 60.00 60.00",
        test_07_floats: "2.5 3.5 4.0" => "38.21 60.00 81.79",
        test_08_small_fractions: "0.3 0.4 0.5" => "36.87 53.13 90.00",
        test_09_obtuse: "4 7 10" => "18.19 33.12 128.68",
        test_10_isosceles_right: "10 10 14.1421356" => "45.00 45.00 90.00",
    }
}
