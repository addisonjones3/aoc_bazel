use crate::day5::parsers::{parse_blocks, parse_range_blocks};
use crate::{
    Solver,
    day5::seed::{CMMap, min_destination},
};
use anyhow::{Error, anyhow};

pub const DAY5_SOLVER: Solver = Solver { part1, part2 };

pub fn part1(input: &str) -> Result<String, Error> {
    let (_, (seeds, cat_maps)) = parse_blocks(input).map_err(|e| anyhow!("{e}"))?;

    let cm_map = CMMap::from_iter(cat_maps);

    let min_destination_val = min_destination(seeds, &cm_map, String::from("seed"))
        .ok_or(anyhow!("didn't find a min value"))?;

    Ok(format!("{min_destination_val}"))
}

pub fn part2(input: &str) -> Result<String, Error> {
    let (_, (seeds, cat_maps)) = parse_range_blocks(input).map_err(|e| anyhow!("{e}"))?;

    let cm_map = CMMap::from_iter(cat_maps);

    let min_destination_val = min_destination(seeds, &cm_map, String::from("seed"))
        .ok_or(anyhow!("didn't find a min value"))?;

    Ok(format!("{min_destination_val}"))
}
