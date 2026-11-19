use std::str::FromStr;

use std::fmt;

#[derive(Debug)]
pub enum Turn {
    Left(i32),
    Right(i32),
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Turn::Left(val) => write!(f, "Left: {}", val),
            Turn::Right(val) => write!(f, "Right: {}", val),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTurnErr;

impl FromStr for Turn {
    type Err = ParseTurnErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let turn_direction = match s.chars().nth(0) {
            Some(turn_direction) => turn_direction,
            None => return Err(ParseTurnErr),
        };

        let turn_val = match s[1..].parse::<i32>() {
            Ok(turn_val) => turn_val,
            Err(_) => return Err(ParseTurnErr),
        };

        match turn_direction {
            'L' => Ok(Turn::Left(turn_val)),
            'R' => Ok(Turn::Right(turn_val)),
            _ => Err(ParseTurnErr),
        }
    }
}

#[derive(Debug)]
pub struct Dial {
    current_position: i32,
    click_count: i32,
    count_positions: i32,
}

impl Dial {
    pub fn new(starting_pos: i32, count_positions: i32) -> Dial {
        Dial {
            current_position: starting_pos,
            click_count: 0,
            count_positions,
        }
    }

    pub fn clicks(&self) -> i32 {
        self.click_count
    }
}

impl fmt::Display for Dial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Dial Position: {}, Click Count: {}",
            self.current_position, self.click_count
        )
    }
}

pub fn turn(d: &mut Dial, turn: Turn) -> (i32, bool) {
    let turn_val: i32 = match turn {
        Turn::Left(turn_val) => -turn_val,
        Turn::Right(turn_val) => turn_val,
    };

    let count_rotations = turn_val.abs() / d.count_positions;
    // crossed 0 backwards or passed over max
    let abs_position = d.current_position + turn_val;
    match abs_position {
        ..0 => d.click_count += 1,
        101.. => d.click_count += 1,
        _ => (),
    }
    d.click_count += count_rotations;

    d.current_position = (d.current_position + turn_val).rem_euclid(d.count_positions);
    if d.current_position == 0 {
        d.click_count += 1;
    }

    (count_rotations, d.current_position == 0)
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::{BufRead, BufReader};

    use super::*;

    #[test]
    fn test_dial_turn_past_max() {
        let mut d = Dial::new(50, 100);
        let turns: Vec<Turn> = vec![Turn::Right(51)];

        turns.into_iter().fold(0, |acc, t| acc + turn(&mut d, t).0);

        assert_eq!(d.clicks(), 1);
    }

    #[test]
    fn test_dial_turn_to_neg_space() {
        let mut d = Dial::new(50, 100);
        let turns: Vec<Turn> = vec![Turn::Left(51)];

        turns.into_iter().fold(0, |acc, t| acc + turn(&mut d, t).0);

        println!("{}", d);
        assert_eq!(d.clicks(), 1);
    }

    #[test]
    fn test_turns_from_str_vec() {
        let f = fs::File::open("src/sample_input.txt").unwrap();

        let reader = BufReader::new(f);
        let mut turns: Vec<Turn> = vec![];

        reader.lines().for_each(|s| {
            let t: Turn = s.unwrap().parse::<Turn>().unwrap();
            turns.push(t);
        });
        for turn in turns {
            println!("{}", turn);
        }
    }
}
