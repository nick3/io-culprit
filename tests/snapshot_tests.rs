use io_culprit::config::Config;
use io_culprit::models::IncidentMeta;
use io_culprit::watchdog;
use io_culprit::watchdog::snapshot::{create_incident_dir, write_meta_json};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_dir() -> std::path::PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("io-culprit-incident-{stamp}"))
}

#[test]
fn creates_incident_directory_and_meta_file() {
    let base = temp_dir();
    let dir = create_incident_dir(&base, "2026-04-18T02:11:30Z").unwrap();
    assert!(dir.exists());
    let meta = IncidentMeta {
        incident_id: dir.file_name().unwrap().to_string_lossy().to_string(),
        start_time: "2026-04-18T02:11:30Z".to_string(),
        device: "sda".to_string(),
        trigger_reason: "critical".to_string(),
        hostname: "vm01".to_string(),
        kernel: "6.1.0".to_string(),
        rounds: 1,
    };
    write_meta_json(&dir, &meta).unwrap();
    let contents = fs::read_to_string(dir.join("meta.json")).unwrap();
    assert!(contents.contains("sda"));
    let _ = fs::remove_dir_all(base);
}

#[test]
fn watchdog_run_with_missing_proc_files_returns_ok_for_now() {
    std::env::set_var("IO_CULPRIT_ONESHOT", "1");
    let config = Config::default();
    let result = watchdog::run(config);
    std::env::remove_var("IO_CULPRIT_ONESHOT");
    assert!(result.is_ok());
}
