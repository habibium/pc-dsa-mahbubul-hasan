use std::io;

fn calculate_area(coords: (isize, isize, isize, isize, isize, isize)) -> f64 {
    let (x1, y1, x2, y2, x3, y3) = coords;
    /*
     * Calculate the area using the shoelace formula
     * 1 | x1, x2, x3, x1 |
     * — |                |
     * 2 | y1, y2, y3, y1 |
     *
     * 1/2 | (x1.y2 + x2.y3 + x3.y1) - (y1.x2 + y2.x3 + y3.x1) |
     *
     */
    ((x1 * y2 + x2 * y3 + x3 * y1) - (y1 * x2 + y2 * x3 + y3 * x1)).abs() as f64 / 2.0
}

fn main() {
    let mut input = String::new();
    println!("Enter 3 coords of the triangle (x1, y1) (x2, y2) (x3, y3):");
    io::stdin().read_line(&mut input).unwrap();

    let coords = input
        .replace(char::is_whitespace, "") // (x1,y1)(x2,y2)(x3,y3)
        .replace(")(", ",") // (x1,y1,x2,y2,x3,y3)
        .replace(['(', ')'], ""); // x1,y1,x2,y2,x3,y3

    let mut coords = coords.split(",").map(|n| n.parse::<isize>().unwrap());

    let area = calculate_area((
        coords.next().unwrap(),
        coords.next().unwrap(),
        coords.next().unwrap(),
        coords.next().unwrap(),
        coords.next().unwrap(),
        coords.next().unwrap(),
    ));

    println!("{:.2}", area);
}

#[cfg(test)]
mod tests {
    use super::*;

    // A helper macro for safe floating-point comparison
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {
            assert!(
                ($a - $b).abs() < f64::EPSILON,
                "assertion failed: `(left !== right)` \n  left: `{:?}`,\n right: `{:?}`",
                $a,
                $b
            );
        };
    }

    #[test]
    fn test_01_standard_right_triangle() {
        // Right triangle in the 1st quadrant: Base 4, Height 3
        assert_approx_eq!(calculate_area((0, 0, 4, 0, 0, 3)), 6.0);
    }

    #[test]
    fn test_02_negative_coordinates() {
        // Triangle spanning across negative coordinate space: Base 4, Height 4
        assert_approx_eq!(calculate_area((-2, -2, 2, -2, 0, 2)), 8.0);
    }

    #[test]
    fn test_03_collinear_points() {
        // Three points forming a straight line (y = x) should have an area of 0
        assert_approx_eq!(calculate_area((1, 1, 2, 2, 3, 3)), 0.0);
    }

    #[test]
    fn test_04_degenerate_triangle() {
        // A triangle where two points are exactly the same
        assert_approx_eq!(calculate_area((1, 1, 1, 1, 4, 5)), 0.0);
    }

    #[test]
    fn test_05_mixed_quadrants() {
        // Points scattered randomly across positive and negative quadrants
        assert_approx_eq!(calculate_area((-3, 4, 5, 2, 1, -6)), 36.0);
    }

    #[test]
    fn test_06_clockwise_winding() {
        // Same as test_01, but the points are given in a clockwise order.
        // This ensures your formula correctly applies an absolute value constraint.
        assert_approx_eq!(calculate_area((0, 0, 0, 3, 4, 0)), 6.0);
    }

    #[test]
    fn test_07_origin() {
        // All points resting at the origin (0, 0)
        assert_approx_eq!(calculate_area((0, 0, 0, 0, 0, 0)), 0.0);
    }

    #[test]
    fn test_08_fractional_area() {
        // Base 3, Height 3 -> Mathematically yields exactly 4.5
        assert_approx_eq!(calculate_area((0, 0, 3, 0, 0, 3)), 4.5);
    }

    #[test]
    fn test_09_large_coordinates() {
        // Testing with large isize coordinates (Base 1000, Height 1000)
        assert_approx_eq!(
            calculate_area((1000, 1000, 2000, 1000, 1000, 2000)),
            500_000.0
        );
    }

    #[test]
    fn test_10_skinny_triangle() {
        // An extremely wide base (200) with a very short height (1)
        assert_approx_eq!(calculate_area((0, 0, 200, 0, 100, 1)), 100.0);
    }
}
