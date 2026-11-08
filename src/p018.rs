//! Problem 18: Maximum path sum I.
//! Largest top-to-bottom sum in a 15-row triangle, stepping to an adjacent number below.
//!
//! Bottom-up: fold each row into the one above it, replacing every entry by itself plus the larger
//! of its two children. O(rows²) rather than 2^(rows-1) paths, and reusable for problem 67.

const TRIANGLE: &str = "
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";

/// Maximum top-to-bottom path sum of a triangle given as whitespace-separated rows.
pub fn max_path_sum(triangle: &str) -> u64 {
    let rows: Vec<Vec<u64>> = triangle
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse().expect("number"))
                .collect()
        })
        .collect();
    let mut best = rows.last().expect("non-empty triangle").clone();
    for row in rows.iter().rev().skip(1) {
        best = row
            .iter()
            .enumerate()
            .map(|(i, &v)| v + best[i].max(best[i + 1]))
            .collect();
    }
    best[0]
}

pub fn solve() -> u64 {
    max_path_sum(TRIANGLE)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// From the statement: 3 / 7 4 / 2 4 6 / 8 5 9 3 has best path 3+7+4+9 = 23.
    #[test]
    fn example() {
        assert_eq!(max_path_sum("3\n7 4\n2 4 6\n8 5 9 3"), 23);
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 1074);
    }
}
