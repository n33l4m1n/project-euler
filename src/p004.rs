//! Problem 4: Largest palindrome product.
//! Find the largest palindrome made from the product of two 3-digit numbers.
//!
//! Search from the top down and prune: once `a × hi` can't beat the best palindrome found, no smaller `a` can either.

fn is_palindrome(n: u64) -> bool {
    let (mut reversed, mut m) = (0, n);
    while m > 0 {
        reversed = reversed * 10 + m % 10;
        m /= 10;
    }
    reversed == n
}

pub fn largest_palindrome_product(digits: u32) -> u64 {
    let lo = 10u64.pow(digits - 1);
    let hi = 10u64.pow(digits) - 1;
    let mut best = 0;
    for a in (lo..=hi).rev() {
        if a * hi <= best {
            break;
        }
        for b in (lo..=a).rev() {
            let p = a * b;
            if p <= best {
                break;
            }
            if is_palindrome(p) {
                best = p;
            }
        }
    }
    best
}

pub fn solve() -> u64 {
    largest_palindrome_product(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(largest_palindrome_product(2), 9_009);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 906_609);
    }
}
