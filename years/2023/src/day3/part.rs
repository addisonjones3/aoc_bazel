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
    begin: usize,
    end: usize,
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
    pub row: u32,
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
                begin: pos.location_offset(),
                end: end.location_offset() - 1,
            },
            row: pos.location_line() - 1,
        })
        .parse(input)
}

pub fn parse_tokens(input: Span) -> IResult<Span, Vec<Token>> {
    many0(preceded(take_while(|c| c == '.'), parse_token)).parse(input)
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
        location: Location { begin: 2, end: 4 },
        kind: Kind::Part(467),
        row: 0,
    };

    const TOK_123: Token = Token {
        location: Location { begin: 9, end: 11 },
        kind: Kind::Part(123),
        row: 0,
    };

    const TOK_STAR: Token = Token {
        location: Location { begin: 14, end: 14 },
        kind: Kind::Symbol('*'),
        row: 0,
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
