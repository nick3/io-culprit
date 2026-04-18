use io_culprit::config::Config;
use io_culprit::watchdog;
use std::path::Path;

fn main() {
    let config = Config::load(Path::new("/etc/io-culprit/config.yaml"));
    if let Err(err) = watchdog::run(config) {
        eprintln!("io-watchdog failed: {err}");
        std::process::exit(1);
    }
}
