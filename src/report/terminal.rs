use crate::models::ReportJson;

pub fn render_terminal_report(report: &ReportJson) -> String {
    let mut lines = vec![
        format!("Incident: {}", report.incident_id),
        format!("Device: {}", report.device),
        format!("Time Range: {}", report.time_range),
        format!("Summary: {}", report.overall_summary),
    ];
    for (index, suspect) in report.suspects.iter().enumerate() {
        lines.push(format!(
            "Suspect #{}: {} [{}] score={} evidence={}",
            index + 1,
            suspect.label,
            suspect.category,
            suspect.score,
            suspect.evidence.join("; ")
        ));
    }
    for step in &report.next_steps {
        lines.push(format!("Next: {step}"));
    }
    lines.join("\n")
}
