use nom::{
    IResult, Parser,
    character::complete::{line_ending, space0},
    multi::many1,
    sequence::pair,
};

pub fn parse_blank_line(input: &str) -> IResult<&str, ()> {
    many1(pair(space0, line_ending)).map(|_| ()).parse(input)
}

#[cfg(test)]
mod test {
    use nom::{character::complete::alpha1, sequence::separated_pair};

    use super::*;

    const ALPHAS_WITH_BLANK_LINE: &str = "AAA

BBB";

    #[test]
    fn test_parse_blank_line() {
        let (rest, _) = separated_pair(alpha1, parse_blank_line, alpha1)
            .parse(ALPHAS_WITH_BLANK_LINE)
            .expect("didn't parse");
        eprintln!("{rest:?}");
    }
}
