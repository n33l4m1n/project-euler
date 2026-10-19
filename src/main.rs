use std::process::exit;
use std::time::Instant;

fn main() {
    let args: Vec<u32> = std::env::args()
        .skip(1)
        .map(|a| {
            a.parse().unwrap_or_else(|_| {
                eprintln!("usage: project-euler [problem ...]");
                exit(2)
            })
        })
        .collect();
    let problems = if args.is_empty() {
        project_euler::SOLVED.to_vec()
    } else {
        args
    };

    for p in problems {
        let start = Instant::now();
        match project_euler::solve(p) {
            Some(answer) => println!("{p:>3}  {answer:>15}  {:>10.2?}", start.elapsed()),
            None => {
                eprintln!("problem {p}: not solved yet");
                exit(1)
            }
        }
    }
}
