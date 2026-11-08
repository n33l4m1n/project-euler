//! Number theory helpers shared across problems.

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

/// All primes `<= limit`, by the sieve of Eratosthenes.
pub fn primes_up_to(limit: usize) -> Vec<u64> {
    if limit < 2 {
        return Vec::new();
    }
    let mut composite = vec![false; limit + 1];
    let mut i = 2;
    while i * i <= limit {
        if !composite[i] {
            for j in (i * i..=limit).step_by(i) {
                composite[j] = true;
            }
        }
        i += 1;
    }
    (2..=limit)
        .filter(|&k| !composite[k])
        .map(|k| k as u64)
        .collect()
}

/// Sum of two non-negative decimal numbers given as digit strings, as a digit string.
/// Schoolbook addition with carry, O(max(len)). Panics on non-digit characters.
pub fn add_decimal(a: &str, b: &str) -> String {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut out = Vec::with_capacity(a.len().max(b.len()) + 1);
    let mut carry = 0;
    for i in 0..a.len().max(b.len()) {
        let digit = |s: &[u8]| {
            s.len()
                .checked_sub(i + 1)
                .map_or(0, |j| (s[j] as char).to_digit(10).expect("digit") as u8)
        };
        let sum = digit(a) + digit(b) + carry;
        out.push(b'0' + sum % 10);
        carry = sum / 10;
    }
    if carry > 0 {
        out.push(b'0' + carry);
    }
    while out.len() > 1 && out.last() == Some(&b'0') {
        out.pop();
    }
    out.reverse();
    String::from_utf8(out).expect("ascii digits")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_addition() {
        assert_eq!(add_decimal("0", "0"), "0");
        assert_eq!(add_decimal("999", "1"), "1000");
        assert_eq!(add_decimal("123", "98765"), "98888");
        assert_eq!(
            add_decimal("99999999999999999999", "99999999999999999999"),
            "199999999999999999998"
        );
    }

    #[test]
    fn gcd_lcm() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(lcm(4, 6), 12);
    }

    #[test]
    fn sieve() {
        assert_eq!(primes_up_to(1), Vec::<u64>::new());
        assert_eq!(primes_up_to(30), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }
}
