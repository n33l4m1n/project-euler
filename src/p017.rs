//! Problem 17: Number letter counts.
//! Letters used writing 1..=1000 in words (British "and"; spaces and hyphens not counted).
//!
//! Each number is spelled by a direct rule (units/teens table, tens table, hundreds with "and")
//! and its letters are counted: O(n) for the range with a constant-size table.

const UNITS: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

/// Number of letters in `n` (1..=1000) written out in words.
pub fn letters(n: usize) -> usize {
    match n {
        1..=19 => UNITS[n].len(),
        20..=99 => TENS[n / 10].len() + UNITS[n % 10].len(),
        100..=999 => {
            let head = UNITS[n / 100].len() + "hundred".len();
            if n.is_multiple_of(100) {
                head
            } else {
                head + "and".len() + letters(n % 100)
            }
        }
        1000 => "onethousand".len(),
        _ => panic!("out of range: {n}"),
    }
}

pub fn solve() -> u64 {
    (1..=1000).map(letters).sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// From the statement: 1..=5 use 19 letters; 342 uses 23; 115 uses 20.
    #[test]
    fn example() {
        assert_eq!((1..=5).map(letters).sum::<usize>(), 19);
        assert_eq!(letters(342), 23);
        assert_eq!(letters(115), 20);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 21124);
    }
}
