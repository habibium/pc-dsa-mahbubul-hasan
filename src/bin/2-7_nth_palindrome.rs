use std::io;

fn is_palindrome(n: usize) -> bool {
    let n = n.to_string();
    let n_reversed: String = n.chars().rev().collect();

    n == n_reversed
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut i = 0;
    let mut palindrome = 0;
    let mut nth = 1;
    while nth <= n {
        if is_palindrome(i) {
            palindrome = i;
            nth += 1;
        }
        i += 1;
    }

    println!("{palindrome}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_digit_numbers() {
        assert!(is_palindrome(1));
        assert!(is_palindrome(2));
        assert!(is_palindrome(3));
        assert!(is_palindrome(9));
    }

    #[test]
    fn double_digit_numbers() {
        assert!(is_palindrome(11));
        assert!(!is_palindrome(10));
        assert!(is_palindrome(22));
        assert!(!is_palindrome(23));
        assert!(is_palindrome(33));
        assert!(!is_palindrome(39));
    }

    #[test]
    fn triple_digit_numbers() {
        assert!(is_palindrome(101));
        assert!(is_palindrome(111));
        assert!(is_palindrome(121));
        assert!(!is_palindrome(100));
        assert!(!is_palindrome(110));
    }
}
