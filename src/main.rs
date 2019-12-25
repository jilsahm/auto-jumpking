#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate regex;
extern crate winapi;

use std::{
    env,
    thread::sleep,
    time::Duration,
};

mod command;
mod error;
mod keyboard;
mod sequence;
mod win;

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

fn main() {
    setup_logger();
    startup_delay(5);
}
