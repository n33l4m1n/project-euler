//! Problem 2: Even Fibonacci numbers.
//! Sum the even-valued Fibonacci terms that do not exceed four million.
//!
//! Every third Fibonacci number is even, and the even ones follow E(n) = 4·E(n−1) + E(n−2), starting 2, 8. So we
//! walk only the even terms and never test parity.

pub fn even_fibonacci_sum(limit: u64) -> u64 {
    let (mut a, mut b) = (2u64, 8u64);
    let mut sum = 0;
    while a <= limit {
        sum += a;
        (a, b) = (b, 4 * b + a);
    }
    sum
}

pub fn solve() -> u64 {
    even_fibonacci_sum(4_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        // Terms up to 89 from the problem statement: the even ones are 2, 8, 34.
        assert_eq!(even_fibonacci_sum(89), 44);
    }

    #[test]
    fn limit_just_below_an_even_term() {
        // 144 is the first term past 100 and it's even. The 2017 C version added it anyway and printed 188.
        assert_eq!(even_fibonacci_sum(100), 44);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 4_613_732);
    }
}
