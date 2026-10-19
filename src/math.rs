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

#[cfg(test)]
mod tests {
    use super::*;

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
