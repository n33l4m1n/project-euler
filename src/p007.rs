//! Problem 7: 10001st prime.
//! What is the 10 001st prime number?
//!
//! Sieve up to Rosser's bound: for n ≥ 6 the nth prime is below n(ln n + ln ln n), so one sieve pass is enough.

use crate::math::primes_up_to;

pub fn nth_prime(n: usize) -> u64 {
    assert!(n >= 1, "primes are 1-indexed");
    let bound = if n < 6 {
        13
    } else {
        let x = n as f64;
        (x * (x.ln() + x.ln().ln())).ceil() as usize
    };
    primes_up_to(bound)[n - 1]
}

pub fn solve() -> u64 {
    nth_prime(10_001)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(nth_prime(6), 13);
        assert_eq!(nth_prime(1), 2);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 104_743);
    }
}
