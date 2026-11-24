pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod parsers;

use hack::Solver;

pub const SOLVERS: &[Solver] = &[
    day1::solve::DAY1_SOLVER,
    day2::solve::DAY2_SOLVER,
    day3::solve::DAY3_SOLVER,
    day4::solve::DAY4_SOLVER,
    day5::solve::DAY5_SOLVER,
    day6::solve::DAY6_SOLVER,
    day7::solve::DAY7_SOLVER,
    day8::solve::DAY8_SOLVER,
    day9::solve::DAY9_SOLVER,
    day10::solve::DAY10_SOLVER,
];
