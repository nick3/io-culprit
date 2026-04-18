use crate::config::Config;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeviceMetrics {
    pub util: f64,
    pub await_ms: f64,
    pub iowait_pct: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TriggerDecision {
    pub triggered: bool,
    pub reason: String,
}

pub fn evaluate_device(
    config: &Config,
    previous: Option<DeviceMetrics>,
    current: DeviceMetrics,
) -> TriggerDecision {
    if current.util >= config.util_critical {
        return TriggerDecision {
            triggered: true,
            reason: "device util exceeded critical threshold".to_string(),
        };
    }

    if let Some(previous) = previous {
        let repeated = previous.util >= config.util_threshold
            && previous.await_ms >= config.await_threshold_ms
            && current.util >= config.util_threshold
            && current.await_ms >= config.await_threshold_ms;
        if repeated {
            return TriggerDecision {
                triggered: true,
                reason: "device util and await exceeded threshold twice".to_string(),
            };
        }

        let iowait_repeated = previous.iowait_pct >= config.iowait_threshold
            && previous.util >= 80.0
            && current.iowait_pct >= config.iowait_threshold
            && current.util >= 80.0;
        if iowait_repeated {
            return TriggerDecision {
                triggered: true,
                reason: "system iowait and device util exceeded threshold twice".to_string(),
            };
        }
    }

    TriggerDecision {
        triggered: false,
        reason: "no trigger".to_string(),
    }
}
