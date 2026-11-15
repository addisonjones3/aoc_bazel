use std::{collections::HashMap, str::FromStr};

use anyhow::{Error, anyhow};

use crate::Solver;

pub const DAY1_SOLVER: Solver = Solver { part1, part2 };

pub fn part1(input: &str) -> Result<String, Error> {
    let calibration_sum: u32 = input
        .lines()
        .map(|s| -> Result<u32, Error> {
            let mut digits = s.chars().filter_map(|c| c.to_digit(10));
            let first = digits
                .next()
                .ok_or_else(|| anyhow!("no digits in line {s:?}"))?;
            let last = digits.next_back().unwrap_or(first);
            Ok(first * 10 + last)
        })
        .sum::<Result<u32, Error>>()?;
    Ok(calibration_sum.to_string())
}

pub fn part2(input: &str) -> Result<String, Error> {
    let digit_names = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let calibration_sum: u32 = input
        .lines()
        .map(|line| -> Result<u32, Error> {
            let digits: Vec<u32> = (0..line.len())
                .filter_map(|i| {
                    let rest = &line[i..];
                    if let Some(num) = rest.chars().next().and_then(|c| c.to_digit(10)) {
                        return Some(num);
                    };

                    None
                })
                .collect();
            Ok(1)
        })
        .sum::<Result<u32, Error>>()?;
    Ok(calibration_sum.to_string())
}
