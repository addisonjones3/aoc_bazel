use super::oasis::*;
use crate::Solver;
use anyhow::{Error, anyhow};

pub const DAY9_SOLVER: Solver = Solver { part1, part2 };

pub fn part1(input: &str) -> Result<String, Error> {
    let (_, rows) = parse_lines(input).map_err(|e| anyhow!("{e}"))?;

    let next_num_sums = rows.iter().map(|row| next_num(row)).sum::<i32>();
    Ok(format!("{next_num_sums}"))
}

pub fn part2(input: &str) -> Result<String, Error> {
    let (_, rows) = parse_lines(input).map_err(|e| anyhow!("{e}"))?;

    let next_num_sums = rows.iter().map(|row| next_num_back(row)).sum::<i32>();
    Ok(format!("{next_num_sums}"))
}
