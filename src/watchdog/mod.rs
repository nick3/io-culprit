pub mod proc;
pub mod trigger;
pub mod snapshot;
pub mod lifecycle;

use crate::config::Config;
use std::thread;
use std::time::Duration;

pub fn run(config: Config) -> Result<(), String> {
    if std::env::var("IO_CULPRIT_ONESHOT").ok().as_deref() == Some("1") {
        return run_once();
    }

    loop {
        run_once()?;
        thread::sleep(Duration::from_secs(config.interval_secs));
    }
}

fn run_once() -> Result<(), String> {
    Ok(())
}
