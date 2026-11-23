use std::iter::successors;

use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{i32 as parse_i32, line_ending, space0},
    combinator::eof,
    multi::separated_list1,
};

fn parse_nums(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space0, parse_i32).parse(input)
}

pub fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(alt((eof, line_ending)), parse_nums).parse(input)
}

fn num_diffs(nums: &[i32]) -> Vec<i32> {
    nums.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>()
}

fn all_nums_same(nums: &[i32]) -> bool {
    nums.iter().all(|n| *n == 0)
}

pub fn next_num(nums: &[i32]) -> i32 {
    successors(Some(nums.to_vec()), |row| {
        if all_nums_same(row) {
            None
        } else {
            Some(num_diffs(row))
        }
    })
    .filter_map(|row| row.last().copied())
    .sum()
}

pub fn next_num_back(nums: &[i32]) -> i32 {
    let diffs: Vec<Vec<i32>> = successors(Some(nums.to_vec()), |row| {
        if all_nums_same(row) {
            None
        } else {
            Some(num_diffs(row))
        }
    })
    .collect();

    diffs
        .iter()
        .rev()
        .fold(0, |acc, row| row.first().unwrap() - acc)
}

#[cfg(test)]
mod test {
    use super::*;

    const SINGLE_DIFF_STR: &str = "0 3 6 9 12 15";
    const DOUBLE_DIFF_STR: &str = "1 3 6 10 15 21";
    const THIRD_DIFF_STR: &str = "10 13 16 21 30 45";

    #[test]
    fn test_num_diffs() {
        let (_, nums) = parse_nums(SINGLE_DIFF_STR).expect("couldn't parse line of nums");

        let diffs = num_diffs(&nums);
        let all_same = diffs.windows(2).all(|w| w[0] == w[1]);
        eprintln!("diffs: {diffs:?}");
        eprintln!("all_same: {all_same:?}");
        assert!(all_same);
    }

    #[test]
    fn test_last_nums() {
        let (_, row) = parse_nums(DOUBLE_DIFF_STR).expect("couldn't parse line of nums");
        let _ = next_num(&row);
    }

    #[test]
    fn test_next_num_back() {
        let (_, row) = parse_nums(THIRD_DIFF_STR).expect("couldn't parse line of nums");
        let next_back = next_num_back(&row);
        assert_eq!(next_back, 5);
    }
}
