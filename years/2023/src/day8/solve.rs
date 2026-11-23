use crate::{
    Solver,
    day8::{
        network::{EndType, Node, node_map_steps_to_final},
        parsers::parse_steps_and_nodes,
    },
};
use anyhow::{Error, anyhow};
use num::integer::lcm;

pub const DAY8_SOLVER: Solver = Solver { part1, part2 };

pub fn part1(input: &str) -> Result<String, Error> {
    let (_, (steps, nm)) = parse_steps_and_nodes(input).map_err(|e| anyhow!("{e}"))?;
    let step_count = node_map_steps_to_final(&steps, &nm, "AAA", EndType::Exact("ZZZ"))?;
    Ok(format!("{step_count}"))
}

pub fn part2(input: &str) -> Result<String, Error> {
    let (_, (steps, nm)) = parse_steps_and_nodes(input).map_err(|e| anyhow!("{e}"))?;
    let nodes_ending_in_a: Vec<&Node> =
        nm.iter().filter(|&node| node.name.ends_with("A")).collect();

    let step_counts: Vec<usize> = nodes_ending_in_a
        .iter()
        .map(|&node| {
            node_map_steps_to_final(&steps, &nm, &node.name, EndType::EndsWith("Z")).unwrap()
        })
        .collect();

    let step_counts_lcm = step_counts.into_iter().reduce(lcm).unwrap_or(1);
    Ok(format!("{step_counts_lcm}"))
}
