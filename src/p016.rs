//! Problem 16: Power digit sum.
//! Sum of the digits of 2^1000.
//!
//! 2^1000 has 302 digits, so it is built by doubling a little-endian digit vector 1000 times with
//! the shared `math::mul_small`: O(n·d) with d ≈ 0.3n digits, trivial for n = 1000.

use crate::math::mul_small;

pub fn power_digit_sum(exp: u32) -> u64 {
    let mut digits = vec![1];
    for _ in 0..exp {
        mul_small(&mut digits, 2);
    }
    digits.iter().map(|&d| u64::from(d)).sum()
}

pub fn solve() -> u64 {
    power_digit_sum(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// From the statement: 2^15 = 32768, digit sum 3+2+7+6+8 = 26.
    #[test]
    fn example() {
        assert_eq!(power_digit_sum(15), 26);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 1366);
    }
}
