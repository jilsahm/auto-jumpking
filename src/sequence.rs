use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::command::Command;
use crate::keyboard::KeyBoard;

pub struct Sequence {
    commands: Vec<Command>,
}

impl Sequence {

    pub fn from_file(path: &Path) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        //let mut commands = Vec::default();

        reader
            .lines()
            .enumerate()
            .filter(|(_, line)| line.is_ok())
            .map(|(i, line)| (i, line.unwrap()))
            .filter(|(_, line)| !line.trim().starts_with("#"))
            .filter(|(_, line)| line.trim().len() > 0);

        unimplemented!()
    }

    fn parse_line(line: &str) -> Vec<Command> {
        unimplemented!()
    }

    pub fn run(&self, keyboard: &KeyBoard) {
        self.commands
            .iter()
            .for_each(|cmd| cmd.trigger(keyboard));
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::Sequence;

    #[test]
    fn test_sequence_from_file_failure() {
        assert!(Sequence::from_file(&PathBuf::from("unknown.lvl")).is_err());
    }
}