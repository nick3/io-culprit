use crate::models::ReportJson;
use std::fs;
use std::path::Path;

pub fn write_report_json(path: &Path, report: &ReportJson) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(report).map_err(|err| err.to_string())?;
    fs::write(path, bytes).map_err(|err| err.to_string())
}
