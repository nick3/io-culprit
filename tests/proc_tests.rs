use io_culprit::models::{IncidentMeta, ReportJson, SuspectFinding};
use io_culprit::watchdog::proc::{parse_cpu_stat_line, parse_diskstats_line};

#[test]
fn report_json_serializes_expected_fields() {
    let _meta = IncidentMeta {
        incident_id: "incident-1".to_string(),
        start_time: "2026-04-18T02:11:30Z".to_string(),
        device: "sda".to_string(),
        trigger_reason: "utilization spike".to_string(),
        hostname: "db-1".to_string(),
        kernel: "6.8.0".to_string(),
        rounds: 3,
    };

    let report = ReportJson {
        incident_id: "incident-1".to_string(),
        time_range: "2026-04-18T02:11:30Z..2026-04-18T02:18:45Z".to_string(),
        device: "sda".to_string(),
        overall_summary: "device saturated".to_string(),
        suspects: vec![SuspectFinding {
            label: "postgres".to_string(),
            category: "userspace-process".to_string(),
            score: 12,
            evidence: vec!["top writer".to_string()],
        }],
        system_findings: vec!["no swap pressure".to_string()],
        next_steps: vec!["inspect postgres logs".to_string()],
        evidence_files: vec!["meta.json".to_string()],
    };

    let json = serde_json::to_string(&report).unwrap();
    assert!(json.contains("incident_id"));
    assert!(json.contains("userspace-process"));
}

#[test]
fn parse_diskstats_line_extracts_expected_fields() {
    let line = "   8       0 sda 157698 233 9514507 64647 226939 190581 33420138 341918 0 144492 406642";
    let sample = parse_diskstats_line(line, 123).unwrap();
    assert_eq!(sample.device, "sda");
    assert_eq!(sample.reads_completed, 157698);
    assert_eq!(sample.writes_completed, 226939);
    assert_eq!(sample.io_ms, 144492);
    assert_eq!(sample.timestamp_secs, 123);
}

#[test]
fn parse_cpu_stat_line_extracts_iowait() {
    let line = "cpu  2255 34 2290 22625563 6290 127 456";
    let sample = parse_cpu_stat_line(line).unwrap();
    assert_eq!(sample.iowait, 6290);
    assert_eq!(sample.system, 2290);
}
