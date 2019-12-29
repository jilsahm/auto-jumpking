#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;
extern crate winapi;

use std::{
    env,
    path::PathBuf,
    thread::sleep,
    time::Duration,
};

mod command;
mod error;
mod keyboard;
mod sequence;
mod win;

use crate::error::ArgsError;
use crate::keyboard::KeyBoard;
use crate::sequence::Sequence;

fn setup_logger() {
    const LOG_VAR: &'static str = "RUST_LOG";
    match env::var(LOG_VAR) {
        Ok(_) => env_logger::init(),
        Err(e) => { 
            env::set_var(LOG_VAR, "info");
            env_logger::init();
            info!("Failed to read environment variable {} because {}, set log level to info", LOG_VAR, e);
        }
    };
}

fn startup_delay(secs: u64) {
    info!("Stating bot in ...");
    (0..secs)
        .for_each(|s| {
            info!("{}", secs - s);
            sleep(Duration::from_secs(1));
        });
}

fn load_sequence() -> Result<Sequence, ArgsError> {
    match env::args().nth(1) {
        Some(arg) => {
            let mut me = PathBuf::from("resources");
            me.push(arg);
            Sequence::from_file(&me).map_err(|_| ArgsError::IoError)
        },
        None => Err(ArgsError::MissingLevelParameter),
    }
}

fn main() {
    setup_logger();
    match KeyBoard::new(&PathBuf::from("resources/keys.cfg")) {
        Ok(keyboard) => {
            info!("Successfully loaded keyboard");
            match load_sequence() {
                Ok(sequence) => {
                    info!("Sequence successfully loaded");
                    startup_delay(5);
                    sequence.run(&keyboard);
                    info!("Finished sequence")
                },
                Err(what) => {
                    warn!("{}", what);
                    info!("Usage: auto-jumpking [levelfilename]");
                },
            }
        },
        Err(what) => error!("Failed to load keyboard because: {}", what),
    }
}
