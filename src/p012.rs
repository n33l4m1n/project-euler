//! Problem 12: Highly divisible triangular number.
//! First triangle number with over five hundred divisors.
//!
//! Walk the triangle numbers and count divisors by trial division up to the square root,
//! O(√T) each. The answer is about 7.6e7 (n ≈ 12375), so this is a few million operations.

pub fn divisor_count(n: u64) -> u64 {
    let mut count = 0;
    let mut d = 1;
    while d * d <= n {
        if n.is_multiple_of(d) {
            count += if d * d == n { 1 } else { 2 };
        }
        d += 1;
    }
    count
}

/// First triangle number with more than `limit` divisors.
pub fn first_triangle_over(limit: u64) -> u64 {
    let (mut n, mut t) = (1, 1);
    while divisor_count(t) <= limit {
        n += 1;
        t += n;
    }
    t
}

pub fn solve() -> u64 {
    first_triangle_over(500)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// From the statement: 28 is the first triangle number with over five divisors.
    #[test]
    fn example() {
        assert_eq!(divisor_count(28), 6);
        assert_eq!(first_triangle_over(5), 28);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 76_576_500);
    }
}
