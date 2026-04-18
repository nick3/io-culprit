use io_culprit::config::Config;
use io_culprit::report;
use std::path::Path;

fn main() {
    let config = Config::load(Path::new("/etc/io-culprit/config.yaml"));
    if let Err(err) = report::run(config) {
        eprintln!("io-report failed: {err}");
        std::process::exit(1);
    }
}
