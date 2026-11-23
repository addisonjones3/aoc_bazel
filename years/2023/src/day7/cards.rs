use anyhow::{Error, anyhow};
use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{anychar, line_ending, space1, usize as parse_usize},
    combinator::{eof, map_res},
    multi::{count, separated_list1},
    sequence::separated_pair,
};
use std::{collections::BTreeMap, fmt::Debug};

const HAND_SIZE: usize = 5;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Card(Type),
    Either(Type, Type),
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Joker => write!(f, "J"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::Ten => write!(f, "10"),
            Self::Jack => write!(f, "J"),
            Self::Queen => write!(f, "Q"),
            Self::King => write!(f, "K"),
            Self::Ace => write!(f, "A"),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::Card(Type::Ace)),
            'K' => Ok(Self::Card(Type::King)),
            'Q' => Ok(Self::Card(Type::Queen)),
            'J' => Ok(Self::Either(Type::Jack, Type::Joker)),
            'T' => Ok(Self::Card(Type::Ten)),
            '9' => Ok(Self::Card(Type::Nine)),
            '8' => Ok(Self::Card(Type::Eight)),
            '7' => Ok(Self::Card(Type::Seven)),
            '6' => Ok(Self::Card(Type::Six)),
            '5' => Ok(Self::Card(Type::Five)),
            '4' => Ok(Self::Card(Type::Four)),
            '3' => Ok(Self::Card(Type::Three)),
            '2' => Ok(Self::Card(Type::Two)),
            _ => Err(anyhow!("unknown card type for {c}")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandKind {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    pub fn from_either(cards: &[Card; HAND_SIZE], either_as: &Type) -> Self {
        let mut card_counts: BTreeMap<&Type, u8> = BTreeMap::default();
        for card in cards {
            let t = match card {
                Card::Card(t) => t,
                Card::Either(a, b) => {
                    if either_as == a {
                        a
                    } else {
                        b
                    }
                }
            };
            card_counts
                .entry(t)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let joker_count = card_counts.remove(&Type::Joker).unwrap_or(0);

        let mut card_counts: Vec<u8> = card_counts.into_values().collect();
        card_counts.sort_unstable();

        match card_counts.last_mut() {
            Some(v) => *v += joker_count,
            None => card_counts.push(joker_count),
        }

        match card_counts.as_slice() {
            [5] => Self::FiveOfAKind,
            [1, 4] => Self::FourOfAKind,
            [2, 3] => Self::FullHouse,
            [1, 1, 3] => Self::ThreeOfAKind,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 1, 2] => Self::Pair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => unreachable!("5 cards always have counts summing to 5"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    either_as: Type,
}

impl Hand {
    pub fn kind_from_either(&self) -> HandKind {
        let mut card_counts: BTreeMap<&Type, u8> = BTreeMap::default();
        for card in &self.cards {
            let t = match card {
                Card::Card(t) => t,
                Card::Either(a, b) => {
                    if &self.either_as == a {
                        a
                    } else {
                        b
                    }
                }
            };
            card_counts
                .entry(t)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let joker_count = card_counts.remove(&Type::Joker).unwrap_or(0);

        let mut card_counts: Vec<u8> = card_counts.into_values().collect();
        card_counts.sort_unstable();

        match card_counts.last_mut() {
            Some(v) => *v += joker_count,
            None => card_counts.push(joker_count),
        }

        match card_counts.as_slice() {
            [5] => HandKind::FiveOfAKind,
            [1, 4] => HandKind::FourOfAKind,
            [2, 3] => HandKind::FullHouse,
            [1, 1, 3] => HandKind::ThreeOfAKind,
            [1, 2, 2] => HandKind::TwoPair,
            [1, 1, 1, 2] => HandKind::Pair,
            [1, 1, 1, 1, 1] => HandKind::HighCard,
            _ => unreachable!("5 cards always have counts summing to 5"),
        }
    }

    fn resolve_cards(&self) -> [Type; HAND_SIZE] {
        self.cards.map(|c| match c {
            Card::Card(c) => c,
            Card::Either(a, b) => {
                if a == self.either_as {
                    a
                } else {
                    b
                }
            }
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind_from_either()
            .cmp(&other.kind_from_either())
            .then_with(|| self.resolve_cards().cmp(&other.resolve_cards()))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Play {
    pub hand: Hand,
    pub bid: usize,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    map_res(anychar, Card::try_from).parse(input)
}

fn parse_cards(input: &str) -> IResult<&str, [Card; HAND_SIZE]> {
    map_res(count(parse_card, HAND_SIZE), |cards| cards.try_into()).parse(input)
}

fn parse_hand(either_as: Type) -> impl FnMut(&str) -> IResult<&str, Hand> {
    move |input| {
        parse_cards
            .map(|cards| Hand { either_as, cards })
            .parse(input)
    }
}

fn parse_play(either_as: Type) -> impl FnMut(&str) -> IResult<&str, Play> {
    move |input| {
        separated_pair(parse_hand(either_as), space1, parse_usize)
            .map(|(hand, bid)| Play { hand, bid })
            .parse(input)
    }
}

pub fn parse_plays(either_as: Type) -> impl FnMut(&str) -> IResult<&str, Vec<Play>> {
    move |input| separated_list1(alt((eof, line_ending)), parse_play(either_as)).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const AAKKQ_STR: &str = "AAKKQ";
    const AAKKQ_BID_STR: &str = "AAKKQ 123";
    const AAKKQ_CARDS: [Card; HAND_SIZE] = [
        Card::Card(Type::Ace),
        Card::Card(Type::Ace),
        Card::Card(Type::King),
        Card::Card(Type::King),
        Card::Card(Type::Queen),
    ];
    const AAKKQ_HAND: Hand = Hand {
        cards: AAKKQ_CARDS,
        either_as: Type::Jack,
    };
    const AAKKQ_PLAY: Play = Play {
        hand: AAKKQ_HAND,
        bid: 123,
    };

    const TTKKQ_BID_STR: &str = "TTKKQ 456";
    const TTKKQ_CARDS: [Card; HAND_SIZE] = [
        Card::Card(Type::Ten),
        Card::Card(Type::Ten),
        Card::Card(Type::King),
        Card::Card(Type::King),
        Card::Card(Type::Queen),
    ];
    const TTKKQ_HAND: Hand = Hand {
        cards: TTKKQ_CARDS,
        either_as: Type::Jack,
    };
    const TTKKQ_PLAY: Play = Play {
        hand: TTKKQ_HAND,
        bid: 456,
    };

    const AAAAA_CARDS: [Card; HAND_SIZE] = [
        Card::Card(Type::Ace),
        Card::Card(Type::Ace),
        Card::Card(Type::Ace),
        Card::Card(Type::Ace),
        Card::Card(Type::Ace),
    ];
    const AAAAA_HAND: Hand = Hand {
        cards: AAAAA_CARDS,
        either_as: Type::Jack,
    };
    const AAQQT_CARDS: [Card; HAND_SIZE] = [
        Card::Card(Type::Ace),
        Card::Card(Type::Ace),
        Card::Card(Type::Queen),
        Card::Card(Type::Queen),
        Card::Card(Type::Ten),
    ];
    const AAQQT_HAND: Hand = Hand {
        cards: AAQQT_CARDS,
        either_as: Type::Jack,
    };
    const TWO_PLAYS: [Play; 2] = [TTKKQ_PLAY, AAKKQ_PLAY];

    #[test]
    fn test_parse_card() {
        let ace_str = "A";
        let (_, got) = parse_card(ace_str).expect("should parse ace");
        assert_eq!(got, Card::Card(Type::Ace));
    }

    #[test]
    fn test_parse_cards() {
        let (_, got) = parse_cards(AAKKQ_STR).expect("should parse str");
        assert_eq!(got, AAKKQ_CARDS);
    }

    #[test]
    fn test_parse_hand() {
        let (_, got) = parse_hand(Type::Jack)
            .parse(AAKKQ_STR)
            .expect("should parse str");
        assert_eq!(got.cards, AAKKQ_CARDS);
        assert_eq!(got.kind_from_either(), HandKind::TwoPair);
    }

    #[test]
    fn test_parse_play() {
        let (_, got) = parse_play(Type::Jack)
            .parse(AAKKQ_BID_STR)
            .expect("should parse str");
        assert_eq!(got, AAKKQ_PLAY);
    }

    #[test]
    fn test_parse_plays_and_sort() {
        let (_, mut got) = parse_plays(Type::Jack)
            .parse(format!("{AAKKQ_BID_STR}\n{TTKKQ_BID_STR}").as_str())
            .expect("should parse str");
        got.sort_unstable();
        assert_eq!(got, &TWO_PLAYS)
    }

    #[test]
    fn test_cmp() {
        assert!(Type::Ten < Type::Ace);
        assert!(AAAAA_HAND > AAKKQ_HAND);
        assert!(AAKKQ_HAND > AAQQT_HAND);
        // assert_eq!(AAKKQ_HAND.kind, AAQQT_HAND.kind);
    }
}
