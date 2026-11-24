use std::iter::successors;

use anyhow::{Error, anyhow};

use hack::Solver;
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{line_ending, usize as parse_usize},
    combinator::eof,
    multi::separated_list0,
};

pub const DAY1_SOLVER: Solver = Solver { part1, part2 };

fn parse_masses(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list0(alt((eof, line_ending)), parse_usize).parse(input)
}

fn calc_fuel(mass: usize) -> usize {
    (mass / 3).saturating_sub(2)
}

fn calc_fuel_for_mass_plus_fuel(mass: usize) -> usize {
    let init_fuel = calc_fuel(mass);
    successors(Some(init_fuel), |&mass| {
        let fuel = calc_fuel(mass);
        if fuel > 0 { Some(fuel) } else { None }
    })
    .sum()
}

pub fn part1(input: &str) -> Result<String, Error> {
    let (_, masses) = parse_masses(input).map_err(|e| anyhow!("{e}"))?;
    let fuel_sum: usize = masses.into_iter().map(calc_fuel).sum();
    Ok(format!("{fuel_sum}"))
}

pub fn part2(input: &str) -> Result<String, Error> {
    let (_, masses) = parse_masses(input).map_err(|e| anyhow!("{e}"))?;
    let fuel_sum: usize = masses.into_iter().map(calc_fuel_for_mass_plus_fuel).sum();
    Ok(format!("{fuel_sum}"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc_fuel() {
        assert_eq!(2, calc_fuel(12));
        assert_eq!(2, calc_fuel(14));
    }

    #[test]
    fn test_calc_fuel_with_fuel() {
        assert_eq!(2, calc_fuel_for_mass_plus_fuel(14));
        assert_eq!(966, calc_fuel_for_mass_plus_fuel(1969));
    }
}
