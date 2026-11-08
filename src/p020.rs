//! Problem 20: Factorial digit sum.
//! Sum of the digits of 100!.
//!
//! Multiplies a little-endian digit vector by 2, 3, …, n with the shared `math::mul_small`:
//! O(n·d) with d ≈ 158 digits for n = 100.

use crate::math::mul_small;

pub fn factorial_digit_sum(n: u32) -> u64 {
    let mut digits = vec![1];
    for k in 2..=n {
        mul_small(&mut digits, k);
    }
    digits.iter().map(|&d| u64::from(d)).sum()
}

pub fn solve() -> u64 {
    factorial_digit_sum(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// From the statement: 10! = 3628800, digit sum 27.
    #[test]
    fn example() {
        assert_eq!(factorial_digit_sum(10), 27);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 648);
    }
}
