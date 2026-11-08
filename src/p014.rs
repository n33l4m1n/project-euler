//! Problem 14: Longest Collatz sequence.
//! Starting number below one million with the longest chain.
//!
//! Memoise chain lengths for values below the limit: each chain runs until it falls to a
//! known length, so the total work is close to linear in the limit.

/// Number of terms in the chain from `start` down to 1.
pub fn chain_length(start: u64) -> u64 {
    let (mut n, mut len) = (start, 1);
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

/// Starting number below `limit` with the longest chain.
pub fn longest_chain_below(limit: usize) -> u64 {
    let mut cache = vec![0u32; limit.max(2)];
    cache[1] = 1;
    let mut best = (1, 1);
    for start in 2..limit {
        let (mut n, mut steps) = (start as u64, 0u32);
        while n >= limit as u64 || cache[n as usize] == 0 {
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
            steps += 1;
        }
        let len = cache[n as usize] + steps;
        cache[start] = len;
        if len > best.1 {
            best = (start as u64, len);
        }
    }
    best.0
}

pub fn solve() -> u64 {
    longest_chain_below(1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// From the statement: 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1 is 10 terms.
    #[test]
    fn example() {
        assert_eq!(chain_length(13), 10);
        assert_eq!(longest_chain_below(14), 9); // 9 has 20 terms, the longest below 14
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 837_799);
    }
}
