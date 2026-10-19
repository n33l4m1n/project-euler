//! Problem 6: Sum square difference.
//! Find the difference between the square of the sum and the sum of the squares of the first one hundred natural
//! numbers.
//!
//! Closed forms: Σk = n(n+1)/2 and Σk² = n(n+1)(2n+1)/6.

pub fn sum_square_difference(n: u64) -> u64 {
    let sum = n * (n + 1) / 2;
    let sum_of_squares = n * (n + 1) * (2 * n + 1) / 6;
    sum * sum - sum_of_squares
}

pub fn solve() -> u64 {
    sum_square_difference(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(sum_square_difference(10), 2_640);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 25_164_150);
    }
}
