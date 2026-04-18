use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DiskSample {
    pub device: String,
    pub reads_completed: u64,
    pub writes_completed: u64,
    pub sectors_read: u64,
    pub sectors_written: u64,
    pub io_ms: u64,
    pub timestamp_secs: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CpuSample {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct IncidentMeta {
    pub incident_id: String,
    pub start_time: String,
    pub device: String,
    pub trigger_reason: String,
    pub hostname: String,
    pub kernel: String,
    pub rounds: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SuspectFinding {
    pub label: String,
    pub category: String,
    pub score: i32,
    pub evidence: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ReportJson {
    pub incident_id: String,
    pub time_range: String,
    pub device: String,
    pub overall_summary: String,
    pub suspects: Vec<SuspectFinding>,
    pub system_findings: Vec<String>,
    pub next_steps: Vec<String>,
    pub evidence_files: Vec<String>,
}
