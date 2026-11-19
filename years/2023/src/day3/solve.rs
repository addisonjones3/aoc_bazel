use std::collections::HashMap;

use anyhow::{Error, anyhow};

use crate::{
    Solver,
    day3::part::{Kind, Span, Token, parse_tokens},
};

pub const DAY3_SOLVER: Solver = Solver { part1, part2 };

pub fn part1(input: &str) -> Result<String, Error> {
    let mut token_map: HashMap<(usize, u32), &Token> = Default::default();
    let tokens: Vec<Token> = input
        .lines()
        .map(|line| {
            parse_tokens(Span::new(line))
                .map(|(_, token)| token)
                .map_err(|e| anyhow!("failed to parse tokens: {e}"))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();

    eprintln!("{tokens:?}");
    let a: HashMap<u32, HashMap<Kind, Vec<Token>>> = HashMap::default();
    // for token in tokens {
    //     a.entry(token.row)
    //         .or_insert_with_key(|_| HashMap::default())
    //         .entry(token.kind)
    //         .or_insert_with_key(|_| Vec::default());
    // }

    Ok(format!("{a:?}"))
}

pub fn part2(input: &str) -> Result<String, Error> {
    todo!()
}
