//! Problem 3: Largest prime factor.
//! What is the largest prime factor of 600851475143?
//!
//! Trial division, dividing each factor out as it's found so `n` shrinks. Once `f² > n`, whatever is left of `n`
//! (if above 1) is itself prime and the largest factor.

pub fn largest_prime_factor(mut n: u64) -> u64 {
    let mut largest = 1;
    let mut f = 2;
    while f * f <= n {
        while n.is_multiple_of(f) {
            largest = f;
            n /= f;
        }
        f += if f == 2 { 1 } else { 2 };
    }
    if n > 1 { n } else { largest }
}

pub fn solve() -> u64 {
    largest_prime_factor(600_851_475_143)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(largest_prime_factor(13_195), 29);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 6_857);
    }
}
