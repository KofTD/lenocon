use std::env::var;
use std::fs::OpenOptions;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use simplelog::{Config, LevelFilter, WriteLogger};

pub fn configure_logger() -> Result<(), std::io::Error> {
    const STALE_AFTER: Duration = Duration::from_hours(24 * 7);
    let path = log_file();

    let is_stale = path
        .metadata()
        .and_then(|m| m.created())
        .ok()
        .and_then(|created| SystemTime::now().duration_since(created).ok())
        .is_some_and(|age| age > STALE_AFTER);

    let mut options = OpenOptions::new();
    options.create(true).write(true);
    if is_stale {
        options.truncate(true);
    } else {
        options.append(true);
    }

    let file = options.open(&path)?;
    WriteLogger::init(LevelFilter::Info, Config::default(), file).map_err(std::io::Error::other)
}

fn log_file() -> PathBuf {
    let base = var("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|_| var("HOME").map(|h| PathBuf::from(h).join(".local/state")))
        .unwrap_or_else(|_| PathBuf::from("/var/log"));
    base.join("lenocon-daemon.log")
}
