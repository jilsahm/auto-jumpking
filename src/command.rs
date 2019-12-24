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

pub struct Move {
    direction: Key,
    range: u64,
}

impl Command for Move {

    fn trigger(&self, keyboard: &KeyBoard) {
        keyboard.key_down(self.direction.clone());
        sleep(Duration::from_millis(self.range));
        keyboard.key_up(self.direction.clone());
    }

    fn pattern() -> &'static Regex {
        lazy_static! {
            static ref pattern: Regex = "^m[rl][0-9]{1-4}$".parse().expect("Check move regex");            
        } 
        &pattern       
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Move::pattern().is_match(s) {
            //TODO
        } else {
            Err("".into())
        }
    }
}

pub struct Jump {
    direction: Key,
    force: u64,
}

impl Command for Jump {
    
    fn trigger(&self, keyboard: &KeyBoard) {
        const LATENCY: u64 = 5;
        keyboard.key_down(Key::Jump);
        sleep(Duration::from_millis(LATENCY));
        keyboard.key_down(self.direction.clone());        
        sleep(Duration::from_millis(self.force));
        keyboard.key_up(Key::Jump);
        sleep(Duration::from_millis(LATENCY));
        keyboard.key_up(self.direction.clone());
    }
}