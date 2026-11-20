use iddqd::{IdOrdItem, IdOrdMap, id_upcast};

#[derive(Debug, PartialEq, Eq)]
pub struct RangeMapper {
    pub source: usize,
    pub destination: usize,
    pub range: usize,
}

impl RangeMapper {
    pub fn map(&self, num: usize) -> usize {
        if (self.source..(self.source + self.range)).contains(&num) {
            return (num - self.source) + self.destination;
        }
        num
    }
    pub fn contains(&self, num: usize) -> bool {
        (self.source..(self.source + self.range + 1)).contains(&num)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CategoryMap {
    pub name: String,
    pub next_name: String,
    pub mappers: Vec<RangeMapper>,
}

impl CategoryMap {
    pub fn map(&self, num: usize) -> usize {
        self.mappers
            .iter()
            .find(|m| m.contains(num))
            .map_or(num, |m| m.map(num))
    }
}

impl IdOrdItem for CategoryMap {
    type Key<'a> = &'a str;

    fn key(&self) -> Self::Key<'_> {
        &self.name
    }

    id_upcast!();
}

pub type CMMap = IdOrdMap<CategoryMap>;

fn walk_destination(num: usize, cm_map: &CMMap, first_name: String) -> usize {
    let mut curr_num = num;
    let mut curr_name = first_name.as_str();

    while let Some(cm) = cm_map.get(curr_name) {
        curr_num = cm.map(curr_num);
        curr_name = cm.next_name.as_str();
    }
    curr_num
}

pub fn min_destination(seeds: Vec<usize>, cm_map: &CMMap, first_name: String) -> Option<usize> {
    seeds
        .iter()
        .copied()
        .map(|seed| walk_destination(seed, cm_map, first_name.clone()))
        .min()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::day5::parsers::parse_blocks;
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

    #[test]
    fn test_range() {
        let got = MAPPER_1.map(99);
        assert_eq!(51, got);

        let got = MAPPER_1.map(1);
        assert_eq!(1, got);
    }

    #[test]
    fn test_cm_map() {
        let cm = CategoryMap {
            name: "".to_string(),
            next_name: "".to_string(),
            mappers: vec![MAPPER_1, MAPPER_2],
        };

        let num1: usize = 99;
        let want1: usize = 51;
        let num2: usize = 53;
        let want2: usize = 55;

        let got1 = cm.map(num1);
        assert_eq!(want1, got1);
        let got2 = cm.map(num2);
        assert_eq!(want2, got2);
    }

    const EXAMPLE_FULL: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15";

    #[test]
    fn test_walk_destination() {
        let (rest, (seeds, cm)) = parse_blocks(EXAMPLE_FULL).expect("couldn't parse");
        assert_eq!("", rest);

        let cm_map = CMMap::from_iter(cm);
        let want = 81;
        eprintln!("{cm_map:?}");
        let got = walk_destination(seeds[0], &cm_map, String::from("seed"));
        assert_eq!(want, got);
    }

    #[test]
    fn test_walk_destinations() {
        let (rest, (seeds, cm)) = parse_blocks(EXAMPLE_FULL).expect("couldn't parse");
        assert_eq!("", rest);

        let cm_map = CMMap::from_iter(cm);
        let destinations: Vec<usize> = seeds
            .iter()
            .copied()
            .map(|seed| walk_destination(seed, &cm_map, String::from("seed")))
            .collect();

        assert_eq!(destinations, vec![81, 53, 57, 52]);
    }

    #[test]
    fn test_min_destination() {
        let (rest, (seeds, cm)) = parse_blocks(EXAMPLE_FULL).expect("couldn't parse");
        assert_eq!("", rest);

        let cm_map = CMMap::from_iter(cm);
        let min_destination =
            min_destination(seeds, &cm_map, String::from("seed")).expect("should be a value");

        assert_eq!(52, min_destination);
    }
}
