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
pub mod p011;
pub mod p012;
pub mod p013;
pub mod p014;
pub mod p015;
pub mod p016;
pub mod p017;
pub mod p018;
pub mod p019;
pub mod p020;
pub mod p021;

/// Problems with a solution in this crate, in order.
pub const SOLVED: &[u32] = &[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
];

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
        11 => p011::solve(),
        12 => p012::solve(),
        13 => p013::solve(),
        14 => p014::solve(),
        15 => p015::solve(),
        16 => p016::solve(),
        17 => p017::solve(),
        18 => p018::solve(),
        19 => p019::solve(),
        20 => p020::solve(),
        21 => p021::solve(),
        _ => return None,
    })
}
