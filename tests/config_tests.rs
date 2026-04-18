use io_culprit::config::Config;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_path(name: &str) -> PathBuf {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    std::env::temp_dir().join(format!("{name}-{stamp}.yaml"))
}

#[test]
fn default_config_has_expected_values() {
    let config = Config::default();
    assert_eq!(config.interval_secs, 15);
    assert_eq!(config.util_threshold, 90.0);
    assert_eq!(config.await_threshold_ms, 50.0);
    assert_eq!(config.util_critical, 98.0);
    assert_eq!(config.iowait_threshold, 25.0);
    assert_eq!(config.consecutive_triggers, 2);
    assert_eq!(config.max_snapshot_rounds, 3);
    assert_eq!(config.retention_days, 30);
    assert_eq!(config.max_incident_size_mb, 50);
    assert_eq!(config.merge_window_secs, 600);
}

#[test]
fn load_config_overrides_defaults() {
    let path = temp_path("io-culprit-config");
    fs::write(
        &path,
        "interval_secs: 10\nutil_threshold: 85\nincident_dir: /tmp/io-culprit\n",
    )
    .unwrap();

    let config = Config::load(&path);
    assert_eq!(config.interval_secs, 10);
    assert_eq!(config.util_threshold, 85.0);
    assert_eq!(config.incident_dir.display().to_string(), "/tmp/io-culprit");

    let _ = fs::remove_file(path);
}
