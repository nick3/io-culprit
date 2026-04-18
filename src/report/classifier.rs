pub fn classify_label(label: &str) -> &'static str {
    if label.starts_with("jbd2") || label.starts_with("kworker") || label.starts_with("flush-") {
        "filesystem-writeback"
    } else if label.starts_with("kswapd") {
        "memory-pressure"
    } else {
        "userspace-process"
    }
}
