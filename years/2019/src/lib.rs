pub mod day1;
pub mod day2;
pub mod puter;
pub mod parsers;

use hack::Solver;

pub const SOLVERS: &[Solver] = &[
    day1::solve::DAY1_SOLVER,
    day2::solve::DAY2_SOLVER
];
