use super::seed::*;
use crate::parsers::parse_blank_line;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space0, space1, u32 as parse_u32},
    combinator::eof,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
};

fn parse_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(
        tag("seeds: "),
        separated_list1(space0, parse_u32),
        line_ending,
    )
    .parse(input)
}

fn parse_range_mapper(input: &str) -> IResult<&str, RangeMapper> {
    (
        parse_u32,
        preceded(space1, parse_u32),
        preceded(space1, parse_u32),
    )
        .map(|(destination, source, range)| RangeMapper {
            source,
            destination,
            range,
        })
        .parse(input)
}

fn parse_range_mapper_block(input: &str) -> IResult<&str, Vec<RangeMapper>> {
    separated_list1(line_ending, parse_range_mapper).parse(input)
}

fn parse_category_map(input: &str) -> IResult<&str, CategoryMap> {
    terminated(
        separated_pair(
            terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:")),
            line_ending,
            parse_range_mapper_block,
        ),
        alt((eof, line_ending)),
    )
    .map(
        |((name, next_name), mappers): ((&str, &str), Vec<RangeMapper>)| CategoryMap {
            name: name.to_string(),
            next_name: next_name.to_string(),
            mappers,
        },
    )
    .parse(input)
}

fn parse_category_maps(input: &str) -> IResult<&str, Vec<CategoryMap>> {
    separated_list0(parse_blank_line, parse_category_map).parse(input)
}

pub fn parse_blocks(input: &str) -> IResult<&str, (Vec<u32>, Vec<CategoryMap>)> {
    separated_pair(parse_seeds, parse_blank_line, parse_category_maps).parse(input)
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    const MAPPER_1: RangeMapper = RangeMapper {
        destination: 50,
        source: 98,
        range: 2,
    };

    const MAPPER_2: RangeMapper = RangeMapper {
        destination: 52,
        source: 50,
        range: 48,
    };

    const MAPPER_BLOCK: &str = "50 98 2
52 50 48";
    const CATMAP_FULL: &str = "seed-to-soil map:
50 98 2
52 50 48";

    const EXAMPLE_FULL: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15";

    fn want_seed_to_soil() -> CategoryMap {
        CategoryMap {
            name: "seed".to_string(),
            next_name: "soil".to_string(),
            mappers: vec![MAPPER_1, MAPPER_2],
        }
    }

    fn want_soil_to_fertilizer() -> CategoryMap {
        CategoryMap {
            name: "soil".to_string(),
            next_name: "fertilizer".to_string(),
            mappers: vec![
                RangeMapper {
                    destination: 0,
                    source: 15,
                    range: 37,
                },
                RangeMapper {
                    destination: 37,
                    source: 52,
                    range: 2,
                },
                RangeMapper {
                    destination: 39,
                    source: 0,
                    range: 15,
                },
            ],
        }
    }

    #[test]
    fn test_range() {
        let got = MAPPER_1.map(99);
        assert_eq!(51, got);

        let got = MAPPER_1.map(1);
        assert_eq!(1, got);
    }

    #[test]
    fn test_range_mapper_block() {
        let want = &[MAPPER_1, MAPPER_2];
        let (_, got) = parse_range_mapper_block(MAPPER_BLOCK).expect("couldn't parse");
        assert_eq!(got, want);
    }

    #[test]
    fn test_parse_category_map() {
        let (_, got) = parse_category_map(CATMAP_FULL).expect("could not parse");
        assert_eq!(got, want_seed_to_soil());
    }

    #[test]
    fn test_parse_full_example() {
        let s_to_s = want_seed_to_soil();
        let s_to_f = want_soil_to_fertilizer();
        let seeds: Vec<u32> = vec![79, 14, 55, 13];
        let (_, (got_seeds, got_cms)) = parse_blocks(EXAMPLE_FULL).expect("could not parse");

        assert_eq!(seeds, got_seeds);
        assert_eq!(got_cms, &[s_to_s, s_to_f]);
    }
}
