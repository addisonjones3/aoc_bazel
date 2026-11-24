use anyhow::{Context, Error, anyhow};
use clap::{Parser, ValueEnum};
use runfiles::Runfiles;

use std::fs;
use std::string::String;

#[derive(Debug, ValueEnum, Clone)]
pub enum Part {
    One,
    Two,
}

fn parse_part(input: &str) -> Result<Part, String> {
    match input
        .parse::<u8>()
        .map_err(|_| String::from("could not parse int from part value"))?
    {
        1 => Ok(Part::One),
        2 => Ok(Part::Two),
        _ => Err(String::from("must be part 1 or 2")),
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub day: u8,

    #[arg(long, value_parser = parse_part)]
    pub part: Part,

    #[arg(long)]
    pub sample: Option<u8>,
}

pub type SolverFn = fn(&str) -> Result<String, Error>;

pub struct Solver {
    pub part1: fn(&str) -> Result<String, Error>,
    pub part2: fn(&str) -> Result<String, Error>,
}

pub fn lookup_solver_fn(args: Args, solvers: &[Solver]) -> Result<SolverFn, Error> {
    let day = args.day as usize;
    day.checked_sub(1)
        .and_then(|i| {
            if let Some(solver) = solvers.get(i) {
                match args.part {
                    Part::One => Some(solver.part1),
                    Part::Two => Some(solver.part2),
                }
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow!("day {day} not implemented"))
}

fn read_input(year: u32, day: u8, sample: Option<u8>) -> Result<String, Error> {
    let r = Runfiles::create().context("couldn't create runfiles")?;

    let file_str = match sample {
        Some(n) => format!("sample{n}.txt"),
        _ => String::from("input.txt"),
    };

    eprintln!("day: {day}, file_str: {file_str}");

    let path = runfiles::rlocation!(r, format!("aoc_files/{year}/day{day}/{file_str}"))
        .ok_or_else(|| anyhow!("couldn't find runfile for day {day}, {file_str}"))?;
    fs::read_to_string(&path).with_context(|| format!("couldn't read {}", path.display()))
}

pub fn run(args: Args, year: u32, solvers: &[Solver]) -> Result<String, Error> {
    let input = read_input(year, args.day, args.sample)?;
    let solver_fn = lookup_solver_fn(args, solvers)?;
    solver_fn(&input)
}
