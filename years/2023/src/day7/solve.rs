use crate::{
    Solver,
    day7::cards::{Type, parse_plays},
};
use anyhow::{Error, anyhow};
use nom::Parser;

pub const DAY7_SOLVER: Solver = Solver { part1, part2 };

pub fn part1(input: &str) -> Result<String, Error> {
    let (_, mut plays) = parse_plays(Type::Jack)
        .parse(input)
        .map_err(|e| anyhow!("{e}"))?;
    plays.sort_unstable();

    Ok(plays
        .into_iter()
        .enumerate()
        .map(|(i, play)| (i + 1) * play.bid)
        .sum::<usize>()
        .to_string())
}

pub fn part2(input: &str) -> Result<String, Error> {
    let (_, mut plays) = parse_plays(Type::Joker)
        .parse(input)
        .map_err(|e| anyhow!("{e}"))?;
    plays.sort_unstable();

    Ok(plays
        .into_iter()
        .enumerate()
        .map(|(i, play)| (i + 1) * play.bid)
        .sum::<usize>()
        .to_string())
}
