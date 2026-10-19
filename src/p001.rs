//! Problem 1: Multiples of 3 or 5.
//! Find the sum of all the multiples of 3 or 5 below 1000.
//!
//! Inclusion–exclusion over three arithmetic series: multiples of 3, plus multiples of 5, minus multiples of 15
//! (counted twice). O(1) instead of the 2017 C version's loop.

fn sum_of_multiples(k: u64, below: u64) -> u64 {
    let n = (below - 1) / k;
    k * n * (n + 1) / 2
}

pub fn multiples_of_3_or_5(below: u64) -> u64 {
    sum_of_multiples(3, below) + sum_of_multiples(5, below) - sum_of_multiples(15, below)
}

pub fn solve() -> u64 {
    multiples_of_3_or_5(1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(multiples_of_3_or_5(10), 23);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 233_168);
    }
}
