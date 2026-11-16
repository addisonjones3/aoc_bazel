use std::collections::HashMap;

use anyhow::{Error, anyhow};

use crate::Solver;
use crate::day2::bag::*;

pub const DAY2_SOLVER: Solver = Solver { part1, part2 };

pub fn part1(input: &str) -> Result<String, Error> {
    let limits = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    let b = Bag::new(limits);

    let games: Vec<Game> = input
        .lines()
        .map(|line| {
            parse_game(line)
                .map(|(_, game)| game)
                .map_err(|e| anyhow!("failed to parse game: {e}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let game_id_sum: usize = games
        .into_iter()
        .filter_map(|g| b.can_play_rounds(g.rounds).then_some(g.id))
        .sum();
    Ok(format!("{game_id_sum}"))
}

pub fn part2(input: &str) -> Result<String, Error> {
    let games: Vec<Game> = input
        .lines()
        .map(|line| {
            parse_game(line)
                .map(|(_, game)| game)
                .map_err(|e| anyhow!("failed to parse game: {e}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let power: usize = games.into_iter().map(|g| Bag::find_limits(g.rounds)).sum();

    Ok(format!("{power}"))
}
