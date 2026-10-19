//! Problem 9: Special Pythagorean triplet.
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.
//!
//! Substitute c = s − a − b into a² + b² = c² and solve for b: b = s(s − 2a) / 2(s − a). One loop over `a`, keeping
//! the values where that division is exact and a < b.

pub fn triplet_product(s: u64) -> Option<u64> {
    (1..s / 3).find_map(|a| {
        let (num, den) = (s * (s - 2 * a), 2 * (s - a));
        let b = num / den;
        (num.is_multiple_of(den) && a < b).then(|| a * b * (s - a - b))
    })
}

pub fn solve() -> u64 {
    triplet_product(1000).expect("the problem guarantees a triplet for 1000")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        // 3² + 4² = 5², and 3 + 4 + 5 = 12.
        assert_eq!(triplet_product(12), Some(60));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 31_875_000);
    }
}
