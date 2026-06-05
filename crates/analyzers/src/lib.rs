pub mod io_uring;
pub mod latency;
pub mod nvme;
pub mod scheduler;

pub struct DeviceLatency {
    pub average_us: u64,
    pub p50_us: u64,
    pub p95_us: u64,
    pub p99_us: u64,
}

pub struct Finding {
    pub severity: Severity,
    pub title: String,
    pub summary: String,
    pub evidence: Vec<String>,
    pub recommendations: Vec<String>,
}

pub enum Severity {
    Low,
    Medium,
    High,
}

pub fn analyze_all() -> Vec<String> {
    vec![
        latency::analyze(),
        scheduler::analyze(),
        nvme::analyze(),
        io_uring::analyze(),
    ]
}
