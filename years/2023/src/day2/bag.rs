use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1, usize as parse_usize},
    combinator::value,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
};
use std::collections::HashMap;

use anyhow::{Error, anyhow};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    pub color: Color,
    pub count: usize,
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

pub type Inventory = HashMap<Color, usize>;

pub struct Bag {
    limits: Inventory,
}

impl Bag {
    pub fn new(limits: Inventory) -> Self {
        Bag { limits }
    }

    pub fn find_limits(rounds: Vec<Round>) -> usize {
        let mut max_map: Inventory = Default::default();
        for round in rounds {
            for mut p in round {
                max_map
                    .entry(p.color)
                    .and_modify(|curr_max| {
                        *curr_max = *curr_max.max(&mut p.count);
                    })
                    .or_insert(p.count);
            }
        }

        max_map.into_values().product()
    }

    pub fn pull(&self, p: &Pull) -> Result<(), Error> {
        let limits = &self.limits;
        if let Some(v) = limits.get(&p.color) {
            v.checked_sub(p.count).ok_or(anyhow!(
                "pull {p:?} would have brought inventory {limits:?} to negative"
            ))?;
        }
        Ok(())
    }

    pub fn can_play_rounds(&self, rounds: Vec<Round>) -> bool {
        for round in rounds {
            for p in round {
                match self.pull(&p) {
                    Ok(_) => {}
                    Err(e) => {
                        let inv = &self.limits;
                        eprintln!("round failed with error {e}");
                        eprintln!("inv is {inv:?}");
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<Round>,
}

pub fn parse_game(input: &str) -> IResult<&str, Game> {
    separated_pair(
        preceded((tag("Game"), space1), parse_usize),
        (tag(":"), space1),
        parse_rounds,
    )
    .map(|(id, rounds)| Game { id, rounds })
    .parse(input)
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

    // TODO: fix later
    //
    // #[test]
    // fn test_game_can_play() {
    //     let game_str = "Game 1: 3 red, 2 blue; 3 red, 2 blue";
    //     let want_game = Game {
    //         id: 1,
    //         rounds: vec![vec![RED_3, BLUE_2], vec![RED_3, BLUE_2]],
    //     };

    //     let (_, got) = parse_game(game_str).expect("expected vec");
    //     assert_eq!(got, want_game);

    //     let b = Bag {
    //         limits: HashMap::from([(Color::Red, 6), (Color::Blue, 4)]),
    //     };
    //     let can_play = b.can_play_rounds(vec![vec![RED_3, BLUE_2], vec![RED_3, BLUE_2]]);
    //     assert!(can_play);

    //     let b2 = Bag {
    //         limits: HashMap::from([(Color::Red, 5), (Color::Blue, 4)]),
    //     };
    //     let cant_play = b2.can_play_rounds(vec![vec![RED_3, BLUE_2], vec![RED_3, BLUE_2]]);
    //     assert!(!cant_play);
    // }
}
