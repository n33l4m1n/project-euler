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

/// Sum of the positive divisors strictly below `n`, using divisor pairs.
/// O(sqrt(n)) time and O(1) space. Returns 0 for `n <= 1`.
pub fn proper_divisor_sum(n: u64) -> u64 {
    if n <= 1 {
        return 0;
    }
    let mut sum = 1;
    let mut divisor = 2;
    while divisor <= n / divisor {
        if n.is_multiple_of(divisor) {
            sum += divisor;
            let paired = n / divisor;
            if paired != divisor {
                sum += paired;
            }
        }
        divisor += 1;
    }
    sum
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

/// Multiplies a little-endian decimal digit vector (`[1]` is 1, `[0, 1]` is 10) by `m` in place.
/// O(len); the carry stays in `u64`, so `m` may be any `u32`. Multiplying by 0 leaves `[0]`.
pub fn mul_small(digits: &mut Vec<u8>, m: u32) {
    let mut carry = 0u64;
    for d in digits.iter_mut() {
        let v = u64::from(*d) * u64::from(m) + carry;
        *d = (v % 10) as u8;
        carry = v / 10;
    }
    while carry > 0 {
        digits.push((carry % 10) as u8);
        carry /= 10;
    }
    while digits.len() > 1 && digits.last() == Some(&0) {
        digits.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proper_divisor_sums() {
        assert_eq!(proper_divisor_sum(0), 0);
        assert_eq!(proper_divisor_sum(1), 0);
        assert_eq!(proper_divisor_sum(2), 1);
        assert_eq!(proper_divisor_sum(6), 6);
        assert_eq!(proper_divisor_sum(28), 28);
        assert_eq!(proper_divisor_sum(12), 16);
        assert_eq!(proper_divisor_sum(36), 55);
    }

    #[test]
    fn small_multiplication() {
        let mut d = vec![5, 2]; // 25
        mul_small(&mut d, 4);
        assert_eq!(d, [0, 0, 1]); // 100
        mul_small(&mut d, 123);
        assert_eq!(d, [0, 0, 3, 2, 1]); // 12300
        mul_small(&mut d, 0);
        assert_eq!(d, [0]);
        let mut d = vec![1];
        mul_small(&mut d, u32::MAX);
        assert_eq!(d, [5, 9, 2, 7, 6, 9, 4, 9, 2, 4]); // 4294967295
    }

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
