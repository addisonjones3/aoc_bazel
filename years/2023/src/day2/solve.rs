use std::collections::HashMap;

use anyhow::{Error, anyhow};

use crate::Solver;

pub const DAY2_SOLVER: Solver = Solver { part1, part2 };

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Blue,
    Red,
    Green,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pull {
    color: Color,
    count: usize,
}

pub struct BagConfig {
    limits: HashMap<Color, usize>,
}

pub struct Game {
    id: usize,
}

pub fn part1(input: &str) -> Result<String, Error> {
    todo!()
}

pub fn part2(input: &str) -> Result<String, Error> {
    todo!()
}
