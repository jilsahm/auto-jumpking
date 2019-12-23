use std::{
    io::{BufRead, BufReader},
    fs::File,
    path::Path,
    str::FromStr,
};

use regex::Regex;
use crate::error::FileError;
use crate::win;

#[derive(Clone, Debug)]
pub enum Key {
    Left,
    Right,
    Jump,
}

impl Key {

    fn len() -> usize {
        3usize
    }

    fn pattern() -> Regex {
        "^(left|right|jump)$".parse::<Regex>().expect("Invalid pattern for config entry")
    }
}

impl FromStr for Key {
    type Err = FileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Key::pattern().captures(s) {
            Some(group) => Ok(
                match group.get(0).map(|m| m.as_str()).unwrap_or_else(|| "jump") {
                    "left" => Key::Left,
                    "right" => Key::Right,
                    "jump" => Key::Jump,
                    _ => unreachable!(),
                }
            ),
            None => Err(FileError::InvalidContentError(format!("{} is an invalid Key", s))),
        }
    }
}

struct Layout {
    keys: Vec<(Key, u16)>,
}

impl Layout {

    fn from_config_file(path: &Path) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let regex = "^[a-z]+=[a-z]+$".parse::<Regex>().expect("Invalid key value pattern");
        let mut keys = Vec::with_capacity(Key::len());

        for line in reader.lines() {
            let line = line.unwrap_or_else(|_| "".into()).trim().to_string();
            if line.len() != 0 && regex.is_match(&line) {
                let entry = line.split("=").collect::<Vec<&str>>();
                match entry[0].parse::<Key>() {
                    Ok(key) => {
                        info!("Found Key {:?} with key {}", key, entry[1]);
                        keys.push((key, win::str_to_keycode(entry[1]).unwrap()));
                    }
                    Err(_e) => error!("TODO"),
                }
            }            
        }

        Ok(Layout { keys, })
    }

    fn code_for(key: &Key) -> u16 {
        unimplemented!()
    }

    fn len(&self) -> usize {
        self.keys.len()
    }
}

pub struct KeyBoard {
    layout: Layout,
}

impl KeyBoard {

    pub fn default() -> Self {
        unimplemented!()
    }

    pub fn key_down(&self, key: Key) {
        unimplemented!()
    }
    
    pub fn key_up(&self, key: Key) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::Layout;
    use std::path::PathBuf;

    fn get_test_config_path() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("ressources/keys.cfg");
        path
    }

    #[test]
    fn test_layout_from_config_file_success() {
        let layout = Layout::from_config_file(&get_test_config_path());
        assert!(layout.is_ok());
        let layout = layout.unwrap();
        assert_eq!(3usize, layout.len());
    }
}