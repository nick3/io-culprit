use crate::models::{CpuSample, DiskSample};
use std::fs;
use std::path::Path;

pub fn parse_diskstats_line(line: &str, timestamp_secs: u64) -> Option<DiskSample> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 14 {
        return None;
    }

    Some(DiskSample {
        device: parts[2].to_string(),
        reads_completed: parts[3].parse().ok()?,
        writes_completed: parts[7].parse().ok()?,
        sectors_read: parts[5].parse().ok()?,
        sectors_written: parts[9].parse().ok()?,
        io_ms: parts[12].parse().ok()?,
        timestamp_secs,
    })
}

pub fn parse_cpu_stat_line(line: &str) -> Option<CpuSample> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 8 || parts[0] != "cpu" {
        return None;
    }

    Some(CpuSample {
        user: parts[1].parse().ok()?,
        nice: parts[2].parse().ok()?,
        system: parts[3].parse().ok()?,
        idle: parts[4].parse().ok()?,
        iowait: parts[5].parse().ok()?,
        irq: parts[6].parse().ok()?,
        softirq: parts[7].parse().ok()?,
        steal: parts.get(8).and_then(|v| v.parse().ok()).unwrap_or_default(),
    })
}

pub fn read_diskstats(path: &Path, timestamp_secs: u64) -> Vec<DiskSample> {
    let Ok(contents) = fs::read_to_string(path) else {
        return Vec::new();
    };

    contents
        .lines()
        .filter_map(|line| parse_diskstats_line(line, timestamp_secs))
        .filter(|sample| !sample.device.starts_with("loop") && !sample.device.starts_with("ram"))
        .collect()
}

pub fn read_cpu_sample(path: &Path) -> Option<CpuSample> {
    let contents = fs::read_to_string(path).ok()?;
    let line = contents.lines().next()?;
    parse_cpu_stat_line(line)
}
