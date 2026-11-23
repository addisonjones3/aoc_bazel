pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod parsers;

use crate::day1::solve::DAY1_SOLVER;
use crate::day2::solve::DAY2_SOLVER;
use crate::day3::solve::DAY3_SOLVER;
use crate::day4::solve::DAY4_SOLVER;
use crate::day5::solve::DAY5_SOLVER;
use crate::day6::solve::DAY6_SOLVER;
use crate::day7::solve::DAY7_SOLVER;
use crate::day8::solve::DAY8_SOLVER;
use crate::day9::solve::DAY9_SOLVER;

use anyhow::{Error, anyhow};
use clap::Parser;

use std::string::String;

pub type SolverFn = fn(&str) -> Result<String, Error>;

pub struct Solver {
    pub part1: fn(&str) -> Result<String, Error>,
    pub part2: fn(&str) -> Result<String, Error>,
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub day: u8,

    #[arg(long)]
    pub part: u8,

    #[arg(long)]
    pub sample: Option<u8>,
}

pub fn get_solver_fn(args: Args) -> Result<SolverFn, Error> {
    let _solver: Solver;
    let _solver_fn: SolverFn;

    let mut _day = 0;
    match args.day {
        1 => {
            _day = {
                _solver = DAY1_SOLVER;
                1
            };
        }
        2 => {
            _day = {
                _solver = DAY2_SOLVER;
                2
            };
        }
        3 => {
            _day = {
                _solver = DAY3_SOLVER;
                3
            };
        }
        4 => {
            _day = {
                _solver = DAY4_SOLVER;
                4
            };
        }
        5 => {
            _day = {
                _solver = DAY5_SOLVER;
                5
            };
        }
        6 => {
            _day = {
                _solver = DAY6_SOLVER;
                6
            };
        }
        7 => {
            _day = {
                _solver = DAY7_SOLVER;
                7
            };
        }
        8 => {
            _day = {
                _solver = DAY8_SOLVER;
                8
            };
        }
        9 => {
            _day = {
                _solver = DAY9_SOLVER;
                9
            };
        }
        _ => return Err(anyhow!("day {:?} is not implemented yet", args.day)),
    }

    let mut _part = 0;
    match args.part {
        1 => {
            _part = {
                _solver_fn = _solver.part1;
                1
            };
        }
        2 => {
            _part = {
                _solver_fn = _solver.part2;
                1
            };
        }
        _ => return Err(anyhow!("part {:?} is invalid", args.part)),
    }

    Ok(_solver_fn)
}
