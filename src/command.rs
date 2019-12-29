use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    time::Duration,
    thread::sleep,
};

use regex::Regex;
use crate::keyboard::{Key, KeyBoard};

#[derive(Debug, PartialEq)]
pub struct Command {
    direction: Direction,
    range: u64,
    jump: bool,
}

impl Command {
    const LATENCY: u64 = 5;
    const WAITTIME: u64 = 25;

    pub fn new(direction: Direction, range: u64, jump: bool) -> Self {
        Self { direction, range, jump, }
    }

    pub fn trigger(&self, keyboard: &KeyBoard) {
        info!("Triggering {}", self);
        if self.jump {
            keyboard.key_down(Key::Jump);
            sleep(Duration::from_millis(Command::LATENCY));
        }
        keyboard.key_down(self.direction.clone().into());        
        sleep(Duration::from_millis(self.range));
        if self.jump {
            keyboard.key_up(Key::Jump);
            sleep(Duration::from_millis(Command::LATENCY));
        }
        keyboard.key_up(self.direction.clone().into());
        if self.jump {
            sleep(Duration::from_millis(self.range));
        } else {
            sleep(Duration::from_millis(Command::WAITTIME));
        }
    }

    fn pattern() -> &'static Regex {
        lazy_static! {
            static ref PATTERN: Regex = "^[jm][rl][0-9]{1,4}$".parse().expect("Check command regex");            
        } 
        &PATTERN  
    }
}

impl Display for Command {

    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            formatter,
            "{}-{:?}-{}",
            if self.jump { "Jump" } else { "Move" },
            self.direction,
            self.range,
        )
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Command::pattern().is_match(s) {
            let direction = if s.contains("r") { Direction::Right } else { Direction::Left };
            let jump = if s.contains("j") { true } else { false };
            let range = s[2..].parse::<u64>().expect("Failed to parse range from move command string");
            Ok(Self { direction, range, jump, })
        } else {
            Err(format!("{} cannot be parsed into a move command. It must match {}", s, Command::pattern()))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::{Command, Direction};

    #[test]
    fn test_command_pattern() {
        vec![
            ("mr100", true),
            ("ml9999", true),
            ("5", false),
            ("", false),
            ("mr", false),
        ]
        .iter()
        .for_each(|(sample, expected)| assert_eq!(*expected, Command::pattern().is_match(sample)));
    }

    #[test]
    fn test_command_parse_success() {
        vec![
            ("ml42", Command::new(Direction::Left, 42, false)),
            ("mr666", Command::new(Direction::Right, 666, false)),
            ("jr666", Command::new(Direction::Right, 666, true)),
            ("jl100", Command::new(Direction::Left, 100, true)),
        ]
        .iter()
        .for_each(|(sample, expected)| assert_eq!(*expected, sample.parse::<Command>().unwrap()));
    }

    #[test]
    fn test_command_parse_failure() {
        vec!["ml42000", "mx666",]
        .iter()
        .for_each(|sample| assert!(sample.parse::<Command>().is_err()));
    }
}