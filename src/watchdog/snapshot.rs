use crate::models::IncidentMeta;
use std::fs;
use std::path::{Path, PathBuf};

pub fn create_incident_dir(base: &Path, start_time: &str) -> Result<PathBuf, String> {
    let stamp = start_time
        .replace([':', '-'], "")
        .replace('T', "-")
        .replace('Z', "");
    let dir = base.join(format!("incident-{stamp}"));
    fs::create_dir_all(&dir).map_err(|err| err.to_string())?;
    Ok(dir)
}

pub fn write_meta_json(dir: &Path, meta: &IncidentMeta) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(meta).map_err(|err| err.to_string())?;
    fs::write(dir.join("meta.json"), bytes).map_err(|err| err.to_string())
}

pub fn write_text_file(dir: &Path, name: &str, contents: &str) -> Result<(), String> {
    fs::write(dir.join(name), contents).map_err(|err| err.to_string())
}
