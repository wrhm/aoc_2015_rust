mod day_01;
mod util;

use std::time::Instant;

// Lint coverage and tests are checked at commit time via
// ../.git/hooks/pre-commit. Hook source:
// https://deaddabe.fr/blog/2021/09/29/git-pre-commit-hook-for-rust-projects/

// Run `cargo test` from ./ (src/) or ../ (aoc_2022/)
// Run `cargo run` from ./ (src/)
fn main() {
    let now = Instant::now();
    day_01::day_01("../data/01.txt");
    let elapsed = now.elapsed();
    println!("Ran all (implemented) solutions in {:?}", elapsed);
}
