use std::fs;

use runfiles::Runfiles;
use rust_2023::day1::solve::DAY1_SOLVER;

fn main() {
    let r = Runfiles::create().unwrap();
    // let path = runfiles::rlocation!(r, "aoc_files/2023/day1/sample1.txt").unwrap();
    let path = runfiles::rlocation!(r, "aoc_files/2023/day1/input.txt").unwrap();
    let input = fs::read_to_string(path).unwrap();
    let answer = (DAY1_SOLVER.part1)(&input).unwrap();
    println!("{:?}", answer);
}
