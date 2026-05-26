use std::env::var;
use std::fs::OpenOptions;
use std::path::PathBuf;

use simplelog::{Config, LevelFilter, WriteLogger};

pub fn configure_logger() -> Result<(), std::io::Error> {
    let log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file())?;
    WriteLogger::init(LevelFilter::Info, Config::default(), log_file)
        .expect("Failed to init logger");
    Ok(())
}

fn log_file() -> PathBuf {
    let base = var("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|_| var("HOME").map(|h| PathBuf::from(h).join(".local/state")))
        .unwrap_or_else(|_| PathBuf::from("/var/log"));
    base.join("lenocon-daemon.log")
}
