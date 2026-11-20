use nom::{
    IResult, Parser,
    character::complete::{line_ending, space0},
    sequence::pair,
};

pub fn parse_blank_line(input: &str) -> IResult<&str, ()> {
    pair(space0, line_ending).map(|_| ()).parse(input)
}
