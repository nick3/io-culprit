pub mod loader;
pub mod scorer;
pub mod classifier;
pub mod terminal;
pub mod json;

use crate::config::Config;
use crate::models::ReportJson;

pub fn run(_config: Config) -> Result<(), String> {
    let report = ReportJson {
        incident_id: "no-incident".to_string(),
        time_range: "unknown".to_string(),
        device: "unknown".to_string(),
        overall_summary: "no incident data".to_string(),
        suspects: Vec::new(),
        system_findings: Vec::new(),
        next_steps: vec!["collect incident data first".to_string()],
        evidence_files: Vec::new(),
    };
    println!("{}", terminal::render_terminal_report(&report));
    Ok(())
}
