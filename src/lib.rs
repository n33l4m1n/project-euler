//! Project Euler solutions. Only problems 1–100 are published, per projecteuler.net's rules.

pub mod math;

pub mod p001;
pub mod p002;
pub mod p003;
pub mod p004;
pub mod p005;
pub mod p006;
pub mod p007;
pub mod p008;
pub mod p009;
pub mod p010;

/// Problems with a solution in this crate, in order.
pub const SOLVED: &[u32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

/// Solves `problem`, or returns `None` if it isn't solved yet.
pub fn solve(problem: u32) -> Option<u64> {
    Some(match problem {
        1 => p001::solve(),
        2 => p002::solve(),
        3 => p003::solve(),
        4 => p004::solve(),
        5 => p005::solve(),
        6 => p006::solve(),
        7 => p007::solve(),
        8 => p008::solve(),
        9 => p009::solve(),
        10 => p010::solve(),
        _ => return None,
    })
}
