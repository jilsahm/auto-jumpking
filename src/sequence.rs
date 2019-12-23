use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::command::{Command, Jump, Move};
use crate::keyboard::KeyBoard;

pub struct Sequence {
    commands: Vec<Box<Command>>,
}

impl Sequence {

    pub fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        unimplemented!()
    }

    pub fn run(&self, keyboard:: &KeyBoard) {
        self.commands
            .iter()
            .for_each(|cmd| cmd.trigger(keyboard));
    }
}