pub mod ebpf;
pub mod perf;
pub mod procfs;
pub mod sysfs;
pub mod tracecmd;

pub struct EvidenceBundle {
    pub sysfs: Option<SysfsSnapshot>,
    pub procfs: Option<ProcfsSnapshot>,
    pub iostat: Option<IostatSnapshot>,
    pub nvme: Option<NvmeSnapshot>,
    pub perf: Option<PerfSnapshot>,
    pub trace: Option<TraceSnapshot>,
}

pub struct SysfsSnapshot {
    pub data: String,
}

pub struct ProcfsSnapshot {
    pub data: String,
}

pub struct IostatSnapshot {
    pub data: String,
}

pub struct NvmeSnapshot {
    pub data: String,
}

pub struct PerfSnapshot {
    pub data: String,
}

pub struct TraceSnapshot {
    pub data: String,
}

pub fn collect_all() -> Vec<String> {
    vec![
        perf::collect(),
        tracecmd::collect(),
        sysfs::collect(),
        procfs::collect(),
        ebpf::collect(),
    ]
}
