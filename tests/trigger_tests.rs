use io_culprit::config::Config;
use io_culprit::watchdog::trigger::{evaluate_device, DeviceMetrics};

#[test]
fn evaluates_consecutive_trigger_rule() {
    let config = Config::default();
    let previous = DeviceMetrics {
        util: 92.0,
        await_ms: 60.0,
        iowait_pct: 30.0,
    };
    let current = DeviceMetrics {
        util: 93.0,
        await_ms: 55.0,
        iowait_pct: 28.0,
    };
    let result = evaluate_device(&config, Some(previous), current);
    assert!(result.triggered);
    assert_eq!(result.reason, "device util and await exceeded threshold twice");
}

#[test]
fn evaluates_single_critical_trigger_rule() {
    let config = Config::default();
    let current = DeviceMetrics {
        util: 99.0,
        await_ms: 10.0,
        iowait_pct: 5.0,
    };
    let result = evaluate_device(&config, None, current);
    assert!(result.triggered);
    assert_eq!(result.reason, "device util exceeded critical threshold");
}
