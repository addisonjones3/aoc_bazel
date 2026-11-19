use std::collections::HashMap;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while,
    character::complete::{none_of, usize as parse_usize},
    multi::many0,
    sequence::preceded,
};
use nom_locate::{LocatedSpan, position as parse_position};

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Location {
    pub x_begin: usize,
    pub x_end: usize,
    pub row: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    Part(usize),
    Symbol(char),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: Kind,
    pub location: Location,
}

pub fn parse_kind(input: Span) -> IResult<Span, Kind> {
    alt((
        parse_usize.map(Kind::Part),
        none_of(".\n").map(Kind::Symbol),
    ))
    .parse(input)
}

pub fn parse_token(input: Span) -> IResult<Span, Token> {
    (parse_position, parse_kind, parse_position)
        .map(|(pos, kind, end)| Token {
            kind,
            location: Location {
                x_begin: pos.location_offset(),
                x_end: end.location_offset() - 1,
                row: pos.location_line() - 1,
            },
        })
        .parse(input)
}

pub fn parse_tokens(input: Span) -> IResult<Span, Vec<Token>> {
    many0(preceded(take_while(|c| c == '.'), parse_token)).parse(input)
}

#[derive(Debug, Default)]
pub struct TokenMap {
    tokens: Vec<Token>,
    map: HashMap<Point, usize>,
}

impl TokenMap {
    pub fn from_vec(tokens: Vec<Token>) -> Self {
        let mut tm: TokenMap = Default::default();
        for (i, token) in tokens.into_iter().enumerate() {
            for x in token.location.x_begin..token.location.x_end {
                tm.map.insert(
                    Point {
                        x,
                        y: token.location.row,
                    },
                    i,
                );
            }
            tm.tokens.insert(i, token);
        }
        tm
    }
    pub fn get(&self, p: &Point) -> Option<&Token> {
        self.map.get(p).and_then(|&i| self.tokens.get(i))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PART_STR: &str = "467";
    const PART: Kind = Kind::Part(467);
    const SYMBOL_STR: &str = "*";
    const SYMBOL: Kind = Kind::Symbol('*');

    const COMBINED_STR: &str = "..467....123..*";
    const TOK_467: Token = Token {
        location: Location {
            x_begin: 2,
            x_end: 4,
            row: 0,
        },
        kind: Kind::Part(467),
    };

    const TOK_123: Token = Token {
        location: Location {
            x_begin: 9,
            x_end: 11,
            row: 0,
        },
        kind: Kind::Part(123),
    };

    const TOK_STAR: Token = Token {
        location: Location {
            x_begin: 14,
            x_end: 14,
            row: 0,
        },
        kind: Kind::Symbol('*'),
    };

    #[test]
    fn test_parse_tokens() {
        let (_, part) = parse_kind(Span::new(PART_STR)).expect("couldn't parse");
        assert_eq!(PART, part);

        let (_, sym) = parse_kind(Span::new(SYMBOL_STR)).expect("couldn't parse");
        assert_eq!(SYMBOL, sym);
    }

    #[test]
    fn test_parse_line() {
        let (_, tokens) = parse_tokens(Span::new(COMBINED_STR)).expect("parse error");
        eprintln!("{tokens:?}");
        assert_eq!(tokens[0], TOK_467);
        assert_eq!(tokens[1], TOK_123);
        assert_eq!(tokens[2], TOK_STAR);
    }
}
