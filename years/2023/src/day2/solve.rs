use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{alpha1, space0, space1, usize as parse_usize},
        streaming::usize,
    },
    combinator::value,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
};
use std::{collections::HashMap, str::FromStr, usize};

use anyhow::{Error, anyhow};

use crate::Solver;

pub const DAY2_SOLVER: Solver = Solver { part1, part2 };

#[derive(Debug, PartialEq, Clone)]
pub enum Color {
    Blue,
    Red,
    Green,
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((
        value(Color::Blue, tag("blue")),
        value(Color::Red, tag("red")),
        value(Color::Green, tag("green")),
    ))
    .parse(input)
}

#[derive(Debug, PartialEq)]
pub struct Pull {
    color: Color,
    count: usize,
}

fn parse_pull(input: &str) -> IResult<&str, Pull> {
    separated_pair(parse_usize, space1, parse_color)
        .map(|(count, color)| Pull { color, count })
        .parse(input)
}

pub type Round = Vec<Pull>;

fn parse_round(input: &str) -> IResult<&str, Round> {
    separated_list0((tag(","), space0), parse_pull).parse(input)
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<Round>> {
    separated_list0((tag(";"), space0), parse_round).parse(input)
}

pub struct Bag {
    limits: HashMap<Color, usize>,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    id: usize,
    rounds: Vec<Round>,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    separated_pair(
        preceded((tag("Game"), space1), parse_usize),
        (tag(":"), space1),
        parse_rounds,
    )
    .map(|(id, rounds)| Game { id, rounds })
    .parse(input)
}

pub fn part1(input: &str) -> Result<String, Error> {
    let games: Vec<Game> = input
        .lines()
        .map(|line| {
            parse_game(line)
                .map(|(_, game)| game)
                .map_err(|e| anyhow!("failed to parse game: {e}"))
        })
        .inspect(|res| eprintln!("{res:?}"))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(format!("{games:?}"))
}

pub fn part2(input: &str) -> Result<String, Error> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const RED_3: Pull = Pull {
        count: 3,
        color: Color::Red,
    };
    const BLUE_2: Pull = Pull {
        count: 2,
        color: Color::Blue,
    };
    const SINGLE_RED_ROUND: [Pull; 1] = [RED_3];
    const ROUND: [Pull; 2] = [RED_3, BLUE_2];

    #[test]
    fn test_parse_color() {
        let red_str = "red";

        let (_, got) = parse_color(red_str).expect("expected red");
        assert_eq!(got, Color::Red);
    }

    #[test]
    fn test_parse_pull() {
        let pull_str = "3 red";
        let (_, got) = parse_pull(pull_str).expect("expected 3 red");
        assert_eq!(got, RED_3);
    }

    #[test]
    fn test_parse_one_pull_round() {
        let round_str = "3 red";
        let (_, got) = parse_round(round_str).expect("expected vec");
        assert_eq!(got, SINGLE_RED_ROUND);
    }

    #[test]
    fn test_parse_round() {
        let round_str = "3 red, 2 blue";
        let want: Round = vec![
            Pull {
                color: Color::Red,
                count: 3,
            },
            Pull {
                color: Color::Blue,
                count: 2,
            },
        ];

        let (_, got) = parse_round(round_str).expect("expected vec");
        assert_eq!(want, got);
    }

    #[test]
    fn test_parse_game() {
        let game_str = "Game 1: 3 red, 2 blue; 3 red, 2 blue";
        let want_game = Game {
            id: 1,
            rounds: vec![vec![RED_3, BLUE_2], vec![RED_3, BLUE_2]],
        };

        let (_, got) = parse_game(game_str).expect("expected vec");
        assert_eq!(got, want_game);
    }
}
