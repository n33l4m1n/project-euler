//! Problem 10: Summation of primes.
//! Find the sum of all the primes below two million.
//!
//! One sieve of Eratosthenes, then sum.

use crate::math::primes_up_to;

pub fn sum_of_primes_below(n: usize) -> u64 {
    primes_up_to(n.saturating_sub(1)).iter().sum()
}

pub fn solve() -> u64 {
    sum_of_primes_below(2_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(sum_of_primes_below(10), 17);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 142_913_828_922);
    }
}
