//! Problem 15: Lattice paths.
//! Routes through a 20×20 grid moving only right and down.
//!
//! Every route is 2n moves of which n go right, so the count is C(2n, n). Computed
//! multiplicatively in O(n); each partial product is itself a binomial, so the division is exact.

pub fn lattice_paths(n: u64) -> u64 {
    (1..=n).fold(1u128, |acc, i| acc * (n + i) as u128 / i as u128) as u64
}

pub fn solve() -> u64 {
    lattice_paths(20)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// From the statement: a 2×2 grid has 6 routes.
    #[test]
    fn example() {
        assert_eq!(lattice_paths(2), 6);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 137_846_528_820);
    }
}
