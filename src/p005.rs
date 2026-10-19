//! Problem 5: Smallest multiple.
//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
//!
//! That's lcm(1, 2, …, 20), folded pairwise.

use crate::math::lcm;

pub fn smallest_multiple(n: u64) -> u64 {
    (1..=n).fold(1, lcm)
}

pub fn solve() -> u64 {
    smallest_multiple(20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(smallest_multiple(10), 2_520);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 232_792_560);
    }
}
