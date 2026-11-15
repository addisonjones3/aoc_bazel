pub mod day1;

use std::string::String;

use anyhow::Error;

pub struct Solver {
    pub part1: fn(&str) -> Result<String, Error>,
    pub part2: fn(&str) -> Result<String, Error>,
}
