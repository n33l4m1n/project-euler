//! Problem 19: Counting Sundays.
//! Sundays falling on the first of a month, 1 Jan 1901 to 31 Dec 2000.
//!
//! No date library, so walk month by month keeping the weekday of the 1st: each month advances it
//! by its length mod 7. Starting from 1 Jan 1900 (a Monday), O(years·12).

const DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Every 4th year, except centuries not divisible by 400.
pub fn is_leap(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

fn month_len(year: u32, month: usize) -> u32 {
    if month == 1 && is_leap(year) {
        29
    } else {
        DAYS[month]
    }
}

/// Weekday of the 1st of `month` (0-based) in `year >= 1900`, with 0 = Monday … 6 = Sunday.
pub fn weekday_of_first(year: u32, month: usize) -> u32 {
    let mut day = 0; // 1 Jan 1900 was a Monday
    for y in 1900..year {
        day += if is_leap(y) { 366 } else { 365 };
    }
    for m in 0..month {
        day += month_len(year, m);
    }
    day % 7
}

pub fn sundays_on_first(from: u32, to: u32) -> u64 {
    let mut weekday = weekday_of_first(from, 0);
    let mut count = 0;
    for year in from..=to {
        for month in 0..12 {
            if weekday == 6 {
                count += 1;
            }
            weekday = (weekday + month_len(year, month)) % 7;
        }
    }
    count
}

pub fn solve() -> u64 {
    sundays_on_first(1901, 2000)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// From the statement: 1 Jan 1900 was a Monday. Hand-checked: 1 Jan 1901 is 365 days later,
    /// a Tuesday; 1 Mar 1900 is 59 days after 1 Jan (1900 not leap), a Thursday; 1 Mar 1904 is
    /// 1460 days (1900–1903, no leap year) + 31 + 29 (leap Feb 1904) = 1520, 1520 mod 7 = 1, a Tuesday.
    #[test]
    fn example() {
        assert_eq!(weekday_of_first(1900, 0), 0);
        assert_eq!(weekday_of_first(1901, 0), 1);
        assert_eq!(weekday_of_first(1900, 2), 3);
        assert_eq!(weekday_of_first(1904, 2), 1);
        assert!(is_leap(2000) && is_leap(1904) && !is_leap(1900) && !is_leap(1901));
    }

    #[test]
    fn answer() {
        assert_eq!(solve(), 171);
    }
}
