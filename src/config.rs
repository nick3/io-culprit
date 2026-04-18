use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub interval_secs: u64,
    pub util_threshold: f64,
    pub await_threshold_ms: f64,
    pub util_critical: f64,
    pub iowait_threshold: f64,
    pub consecutive_triggers: u32,
    pub max_snapshot_rounds: u32,
    pub incident_dir: PathBuf,
    pub retention_days: u32,
    pub max_incident_size_mb: u64,
    pub merge_window_secs: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            interval_secs: 15,
            util_threshold: 90.0,
            await_threshold_ms: 50.0,
            util_critical: 98.0,
            iowait_threshold: 25.0,
            consecutive_triggers: 2,
            max_snapshot_rounds: 3,
            incident_dir: PathBuf::from("/var/log/io-culprit"),
            retention_days: 30,
            max_incident_size_mb: 50,
            merge_window_secs: 600,
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Self {
        let mut config = Self::default();
        let Ok(contents) = fs::read_to_string(path) else {
            return config;
        };

        let values = parse_simple_yaml(&contents);
        if let Some(value) = values.get("interval_secs").and_then(|v| v.parse().ok()) {
            config.interval_secs = value;
        }
        if let Some(value) = values.get("util_threshold").and_then(|v| v.parse().ok()) {
            config.util_threshold = value;
        }
        if let Some(value) = values.get("await_threshold_ms").and_then(|v| v.parse().ok()) {
            config.await_threshold_ms = value;
        }
        if let Some(value) = values.get("util_critical").and_then(|v| v.parse().ok()) {
            config.util_critical = value;
        }
        if let Some(value) = values.get("iowait_threshold").and_then(|v| v.parse().ok()) {
            config.iowait_threshold = value;
        }
        if let Some(value) = values.get("consecutive_triggers").and_then(|v| v.parse().ok()) {
            config.consecutive_triggers = value;
        }
        if let Some(value) = values.get("max_snapshot_rounds").and_then(|v| v.parse().ok()) {
            config.max_snapshot_rounds = value;
        }
        if let Some(value) = values.get("incident_dir") {
            config.incident_dir = PathBuf::from(value);
        }
        if let Some(value) = values.get("retention_days").and_then(|v| v.parse().ok()) {
            config.retention_days = value;
        }
        if let Some(value) = values.get("max_incident_size_mb").and_then(|v| v.parse().ok()) {
            config.max_incident_size_mb = value;
        }
        if let Some(value) = values.get("merge_window_secs").and_then(|v| v.parse().ok()) {
            config.merge_window_secs = value;
        }

        config
    }
}

fn parse_simple_yaml(contents: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once(':') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    map
}
