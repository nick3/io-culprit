use io_culprit::models::{ReportJson, SuspectFinding};
use io_culprit::report::classifier::classify_label;
use io_culprit::report::scorer::score_candidate;
use io_culprit::report::terminal::render_terminal_report;

#[test]
fn classifier_marks_kernel_threads() {
    assert_eq!(classify_label("jbd2/sda1-8"), "filesystem-writeback");
    assert_eq!(classify_label("postgres"), "userspace-process");
}

#[test]
fn scorer_rewards_repeat_top_writer() {
    let score = score_candidate(3, true, true, true);
    assert_eq!(score, 20); // 3*4=12, +3 repeated, +3 aligned, +2 high_cpu = 20
}

#[test]
fn terminal_report_contains_top_suspect() {
    let report = ReportJson {
        incident_id: "incident-1".to_string(),
        time_range: "range".to_string(),
        device: "sda".to_string(),
        overall_summary: "summary".to_string(),
        suspects: vec![SuspectFinding {
            label: "postgres".to_string(),
            category: "userspace-process".to_string(),
            score: 12,
            evidence: vec!["continuous writer".to_string()],
        }],
        system_findings: vec![],
        next_steps: vec!["check postgres logs".to_string()],
        evidence_files: vec!["meta.json".to_string()],
    };
    let text = render_terminal_report(&report);
    assert!(text.contains("Suspect #1: postgres"));
    assert!(text.contains("check postgres logs"));
}
