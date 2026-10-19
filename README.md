# Project Euler

Solutions to [Project Euler](https://projecteuler.net/) problems in Rust, from the N33L4M1N lab notebook.

Each problem lives in `src/pNNN.rs` with a short note on the approach and two tests: the worked example from the
problem statement, and the verified answer. Shared number theory (gcd, lcm, sieve) is in `src/math.rs`.

```sh
cargo test                      # check every answer
cargo run --release             # solve everything, with timings
cargo run --release -- 7 10     # solve specific problems
```

Problems 1 and 2 were first solved in C in 2017. Those originals are kept in [`c/`](c/).

Project Euler asks that solutions beyond the first 100 problems stay private, so this repository stops at
problem 100.

Problem statements are paraphrased from projecteuler.net, licensed
[CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/).
