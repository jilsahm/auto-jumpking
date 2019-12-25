use std::{
    str::FromStr,
    time::Duration,
    thread::sleep,
};

use regex::Regex;
use crate::keyboard::{Key, KeyBoard};

pub trait Command: FromStr {
    fn trigger(&self, keyboard: &KeyBoard);
    fn pattern() -> &'static Regex;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct Move {
    direction: Direction,
    range: u64,
}

impl Move {

    pub fn new(direction: Direction, range: u64) -> Self {
        Self { direction, range, }
    }
}

impl Command for Move {

    fn trigger(&self, keyboard: &KeyBoard) {
        keyboard.key_down(self.direction.clone().into());
        sleep(Duration::from_millis(self.range));
        keyboard.key_up(self.direction.clone().into());
    }

    fn pattern() -> &'static Regex {
        lazy_static! {
            static ref PATTERN: Regex = "^m[rl][0-9]{1,4}$".parse().expect("Check move regex");            
        } 
        &PATTERN       
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Move::pattern().is_match(s) {
            let direction = if s.contains("r") { Direction::Right } else { Direction::Left };
            let range = s[2..].parse::<u64>().expect("Failed to parse range from move command string");
            Ok(Self { direction, range, })
        } else {
            Err(format!("{} cannot be parsed into a move command. It must match {}", s, Move::pattern()))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Jump {
    direction: Direction,
    force: u64,
}

impl Jump {

    pub fn new(direction: Direction, force: u64) -> Self {
        Self { direction, force, }
    }
}

impl Command for Jump {
    
    fn trigger(&self, keyboard: &KeyBoard) {
        const LATENCY: u64 = 5;
        keyboard.key_down(Key::Jump);
        sleep(Duration::from_millis(LATENCY));
        keyboard.key_down(self.direction.clone().into());        
        sleep(Duration::from_millis(self.force));
        keyboard.key_up(Key::Jump);
        sleep(Duration::from_millis(LATENCY));
        keyboard.key_up(self.direction.clone().into());
    }

    fn pattern() -> &'static Regex {
        lazy_static! {
            static ref PATTERN: Regex = "^j[rl][0-9]{1,4}$".parse().expect("Check jump regex");            
        } 
        &PATTERN   
    }
}

impl FromStr for Jump {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Jump::pattern().is_match(s) {
            let direction = if s.contains("r") { Direction::Right } else { Direction::Left };
            let force = s[2..].parse::<u64>().expect("Failed to parse force from jump command string");
            Ok(Self { direction, force, })
        } else {
            Err(format!("{} cannot be parsed into a jump command. It must match {}", s, Move::pattern()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, Direction, Jump, Move};

    #[test]
    fn test_move_pattern() {
        vec![
            ("mr100", true),
            ("ml9999", true),
            ("5", false),
            ("", false),
            ("mr", false),
        ]
        .iter()
        .for_each(|(sample, expected)| assert_eq!(*expected, Move::pattern().is_match(sample)));
    }

    #[test]
    fn test_move_parse_success() {
        vec![
            ("ml42", Move::new(Direction::Left, 42)),
            ("mr666", Move::new(Direction::Right, 666)),
        ]
        .iter()
        .for_each(|(sample, expected)| assert_eq!(*expected, sample.parse::<Move>().unwrap()));
    }

    #[test]
    fn test_move_parse_failure() {
        vec!["ml42000", "mx666",]
        .iter()
        .for_each(|sample| assert!(sample.parse::<Move>().is_err()));
    }

    #[test]
    fn test_jump_pattern() {
        vec![
            ("jr100", true),
            ("jl9999", true),
            ("5", false),
            ("", false),
            ("jr", false),
        ]
        .iter()
        .for_each(|(sample, expected)| assert_eq!(*expected, Jump::pattern().is_match(sample)));
    }

    #[test]
    fn test_jump_parse_success() {
        vec![
            ("jl42", Jump::new(Direction::Left, 42)),
            ("jr666", Jump::new(Direction::Right, 666)),
        ]
        .iter()
        .for_each(|(sample, expected)| assert_eq!(*expected, sample.parse::<Jump>().unwrap()));
    }

    #[test]
    fn test_jump_parse_failure() {
        vec!["jl42000", "jx666",]
        .iter()
        .for_each(|sample| assert!(sample.parse::<Jump>().is_err()));
    }
}