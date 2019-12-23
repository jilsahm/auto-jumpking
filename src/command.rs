use std::{
    time::Duration,
    thread::sleep,
};

use crate::keyboard::{Key, KeyBoard};

pub trait Command {
    fn trigger(&self, keyboard: &KeyBoard);
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