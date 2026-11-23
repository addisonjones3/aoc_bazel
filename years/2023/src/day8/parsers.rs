use super::network::{Node, NodeMap, Step};
use crate::parsers::parse_blank_line;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, one_of},
    multi::{many0, many1},
    sequence::{delimited, separated_pair},
};

fn parse_mappings(input: &str) -> IResult<&str, (String, String)> {
    delimited(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        alt((tag(")\n"), tag(")"))),
    )
    .map(|(left, right): (&str, &str)| (left.to_string(), right.to_string()))
    .parse(input)
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    separated_pair(alphanumeric1, tag(" = "), parse_mappings)
        .map(|(name, (left_name, right_name))| Node {
            name: name.to_string(),
            left_name,
            right_name,
        })
        .parse(input)
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    many0(parse_node).parse(input)
}

fn parse_steps(input: &str) -> IResult<&str, Vec<Step>> {
    many1(one_of("LR"))
        .map(|chars| {
            chars
                .iter()
                .map(|c| match c {
                    'L' => Step::Left,
                    _ => Step::Right,
                })
                .collect::<Vec<Step>>()
        })
        .parse(input)
}

pub fn parse_steps_and_nodes(input: &str) -> IResult<&str, (Vec<Step>, NodeMap)> {
    separated_pair(parse_steps, parse_blank_line, parse_nodes)
        .map(|(steps, nodes)| (steps, NodeMap::from_iter(nodes)))
        .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const LLR_STR: &str = "LLR";
    const NODE_STR: &str = "AAA = (BBB, BBB)";
    const NODES_STR: &str = "AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    const INPUT_STR: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_parse_steps() {
        let (_, got) = parse_steps(LLR_STR).expect("couldn't parse steps");
        assert_eq!(got, [Step::Left, Step::Left, Step::Right]);
    }

    #[test]
    fn test_parse_node() {
        let (_, got) = parse_node(NODE_STR).expect("couldn't parse node");
        assert_eq!(
            got,
            Node {
                name: String::from("AAA"),
                left_name: String::from("BBB"),
                right_name: String::from("BBB")
            }
        );
    }

    #[test]
    fn test_nodes_parse() {
        let (rest, got) = parse_nodes(NODES_STR).expect("couldn't parse nodes");
        eprintln!("rest: {rest:?}");
        let want = vec![
            Node {
                name: String::from("AAA"),
                left_name: String::from("BBB"),
                right_name: String::from("BBB"),
            },
            Node {
                name: String::from("BBB"),
                left_name: String::from("AAA"),
                right_name: String::from("ZZZ"),
            },
            Node {
                name: String::from("ZZZ"),
                left_name: String::from("ZZZ"),
                right_name: String::from("ZZZ"),
            },
        ];
        assert_eq!(got, want);
    }

    #[test]
    fn test_parse_node_map() {
        // let input = format!("{LLR_STR}\n\n{NODES_STR}");
        let (rest, (got_steps, got_map)) =
            parse_steps_and_nodes(INPUT_STR).expect("couldn't parse steps and nodes");
        eprintln!("rest: {rest:?}");
        let want_map = NodeMap::from_iter([
            Node {
                name: String::from("AAA"),
                left_name: String::from("BBB"),
                right_name: String::from("BBB"),
            },
            Node {
                name: String::from("BBB"),
                left_name: String::from("AAA"),
                right_name: String::from("ZZZ"),
            },
            Node {
                name: String::from("ZZZ"),
                left_name: String::from("ZZZ"),
                right_name: String::from("ZZZ"),
            },
        ]);
        assert_eq!(got_steps, [Step::Left, Step::Left, Step::Right]);
        assert_eq!(got_map, want_map);
    }
}
