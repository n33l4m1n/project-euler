//! Problem 21: Amicable numbers.
//! Check divisor sums in pairs with the shared helper, excluding perfect numbers.
//! Trial division is sufficient for this small limit and also checks partners above it.
//! O(L * sqrt(M)) time and O(1) space, where L is the limit and M is the largest
//! candidate or partner checked.

use crate::math::proper_divisor_sum;

fn is_amicable(n: u64) -> bool {
    let partner = proper_divisor_sum(n);
    partner != n && partner > 0 && proper_divisor_sum(partner) == n
}

pub fn solve() -> u64 {
    (2..10_000).filter(|&n| is_amicable(n)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(proper_divisor_sum(220), 284);
        assert_eq!(proper_divisor_sum(284), 220);
        assert!(is_amicable(220));
        assert!(is_amicable(284));
        assert!(!is_amicable(6));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 31_626);
    }
}
