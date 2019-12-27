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
        let mut commands: Vec<Command> = Vec::default();

        reader
            .lines()
            .enumerate()
            .filter(|(_, line)| line.is_ok())
            .map(|(i, line)| (i, line.unwrap()))
            .filter(Sequence::is_no_comment)
            .filter(Sequence::is_not_empty)
            .map(Sequence::parse_line)
            .for_each(|mut seq| commands.append(&mut seq));

        Ok(Self { commands, })
    }

    fn is_no_comment(line: &(usize, String)) -> bool {
        match line.1.trim().starts_with("#") {
            true => {
                debug!("Ignoring line {} because it is a comment", line.0);
                false
            },
            _ => true,
        }        
    }

    fn is_not_empty(line: &(usize, String)) -> bool {
        if line.1.trim().is_empty() {
            debug!("Ignoring line {} because it is empty", line.0);
            false
        } else {
            true
        }
    }

    fn parse_line(line: (usize, String)) -> Vec<Command> {
        line.1
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.trim())
            .map(|s| s.parse::<Command>())
            .filter(|cmd| {
                match cmd {
                    Ok(_) => true,
                    Err(e) => {
                        warn!("Command in line {} could not be parsed because: {}", line.0, e);
                        false
                    }
                }
            })
            .map(|cmd| cmd.unwrap())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.commands.len()
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

    fn get_testfile_path() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test.lvl");
        path
    }

    #[test]
    fn test_sequence_from_file_success() {
        let s = Sequence::from_file(&get_testfile_path());
        assert!(s.is_ok());
        assert_eq!(5usize, s.unwrap().len());
    }

    #[test]
    fn test_sequence_from_file_failure() {
        assert!(Sequence::from_file(&PathBuf::from("unknown.lvl")).is_err());
    }

    #[test]
    fn test_sequence_is_no_comment() {
        vec![
            ((0usize, "# im a comment".to_string()), false),
            ((0usize, "  # me too".to_string()), false),
            ((0usize, "me not".to_string()), true),
            ((0usize, "".to_string()), true),
        ]
        .iter()
        .for_each(|(sample, expected)| assert_eq!(*expected, Sequence::is_no_comment(sample)));
    }

    #[test]
    fn test_sequence_is_not_empty() {
        vec![
            ((0usize, "".to_string()), false),
            ((0usize, "  ".to_string()), false),
            ((0usize, "\t".to_string()), false),
            ((0usize, "\n".to_string()), false),
            ((0usize, "abc\n".to_string()), true),
        ]
        .iter()
        .for_each(|(sample, expected)| assert_eq!(*expected, Sequence::is_not_empty(sample)));
    }

    #[test]
    fn test_sequence_parse_line() {
        vec![
            ("", 0),
            (",", 0),
            ("jr100", 1),
            ("jr100,", 1),
            ("jr100,ml400", 2),
            (" jr100, ml400\n", 2),
        ]
        .iter()
        .for_each(|(sample, expected)| assert_eq!(*expected, Sequence::parse_line((0usize, sample.to_string())).len()));
    }
}