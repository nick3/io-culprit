use crate::models::IncidentMeta;
use std::fs;
use std::path::Path;

pub fn load_meta(dir: &Path) -> Result<IncidentMeta, String> {
    let bytes = fs::read(dir.join("meta.json")).map_err(|err| err.to_string())?;
    serde_json::from_slice(&bytes).map_err(|err| err.to_string())
}
