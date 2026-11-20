use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{space0, space1, u32 as parse_u32},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(tag("seeds: "), separated_list1(space0, parse_u32)).parse(input)
}

pub struct RangeMapper {
    source: u32,
    destination: u32,
    range: u32,
}

impl RangeMapper {
    pub fn map(&self, num: u32) -> u32 {
        match (self.source..(self.source + self.range))
            .contains(&num)
            .then_some(num)
        {
            Some(num) => (num - self.source) + self.destination,
            None => num,
        }
    }
}

pub struct CatMap {}

#[cfg(test)]
mod test {
    use super::*;
    const MAPPER_1: RangeMapper = {source:  }

    #[test]
    fn test_range() {
        let got = 
    }
}
