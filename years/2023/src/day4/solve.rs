use anyhow::{Error, anyhow};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{space0, space1, u32 as parse_u32},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, separated_pair},
};
use std::collections::{
    HashMap, HashSet,
    hash_map::Entry::{Occupied, Vacant},
};

use crate::Solver;

pub const DAY4_SOLVER: Solver = Solver { part1, part2 };

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    id: u32,
    nums: HashSet<u32>,
}

impl Card {
    pub fn calc_hits(&self, nums: &Vec<u32>) -> u32 {
        let mut hits = 0;
        for num in nums {
            if self.nums.contains(num) {
                hits += 1;
            }
        }
        hits
    }
    pub fn calc_score(&self, nums: &Vec<u32>) -> u32 {
        let hits = self.calc_hits(nums);

        match hits {
            0 => 0,
            1.. => 2_u32.pow(hits - 1),
        }
    }
}

fn parse_winning_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
    delimited(space0, separated_list1(space1, parse_u32), space0)
        .map(HashSet::from_iter)
        .parse(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    separated_pair(
        preceded((tag("Card"), space1), parse_u32),
        tag(":"),
        parse_winning_numbers,
    )
    .map(|(id, nums)| Card { id, nums })
    .parse(input)
}

fn parse_line(input: &str) -> Result<(Card, Vec<u32>), Error> {
    separated_pair(
        parse_card,
        delimited(space0, tag("|"), space0),
        separated_list1(space1, parse_u32),
    )
    .parse(input)
    .map(|(_, pair)| pair)
    .map_err(|e| anyhow!("error parsing line: {e}"))
}

pub fn part1(input: &str) -> Result<String, Error> {
    let pairs: Vec<(Card, Vec<u32>)> = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(pairs
        .iter()
        .fold(0_u32, |acc, (card, nums)| acc + card.calc_score(nums))
        .to_string())
}

pub fn part2(input: &str) -> Result<String, Error> {
    let pairs: Vec<(Card, Vec<u32>)> = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?;

    let mut copy_counts: HashMap<u32, u32> = HashMap::new();

    for (card, nums) in pairs.iter() {
        let card_id = card.id;
        let hits = card.calc_hits(nums);
        let curr_copies = copy_counts.get(&card_id).copied().unwrap_or(1);
        (card_id..(card_id + hits + 1).min((pairs.len() + 1) as u32))
            .map(|i| {
                // eprintln!("card {card_id} adding 1 to {i}");
                // match copy_counts.entry(i) {
                //     Occupied(mut o) => {
                //         eprintln!("found entry for {o:?}");
                //         let v = o.get_mut();
                //         *v += 1;
                //         eprintln!("val now {v}");
                //     }
                //     Vacant(v) => {
                //         eprintln!("creating entry for {v:?}");
                //         v.insert(1);
                //     }
                // }
                for j in 0..curr_copies {
                    copy_counts.entry(i).and_modify(|v| *v += 1).or_insert(1);
                }
            })
            .collect()
    }

    eprintln!("{copy_counts:?}");
    Ok(copy_counts.values().sum::<u32>().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const FULL_LINE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

    #[test]
    fn test_parse_u32() {
        let u32_str = "1 2 3";
        let want: HashSet<u32> = HashSet::from([1, 2, 3]);
        let (_, got) = parse_winning_numbers(u32_str).expect("could not parse");
        assert_eq!(want, got);
    }

    #[test]
    fn test_parse_card() {
        let nums: HashSet<u32> = HashSet::from([41, 48, 83, 86, 17]);
        let want = Card { id: 1, nums };
        let (_, card) = parse_card(FULL_LINE).expect("could not parse");
        assert_eq!(card, want);
    }

    #[test]
    fn test_parse_line() {
        let (card, nums) = parse_line(FULL_LINE).expect("couldn't parse");
        let score = card.calc_score(&nums);
        eprintln!("{score}");
    }
}
