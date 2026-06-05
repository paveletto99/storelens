use std::process::Command;
/// Kernel version
/// Root privileges
/// TraceFS availability
/// perf availability
/// trace-cmd availability
/// nvme-cli availability
/// Available tracepoints
/// Available devices

pub struct EnvCheckResult {
    pub kernel_version: String,
    pub has_root: bool,
    pub has_tracefs: bool,
    pub has_perf: bool,
    pub has_trace_cmd: bool,
    pub has_nvme_cli: bool,
}

pub fn check() -> EnvCheckResult {
    let kernel_version = get_kernel_version();
    let has_root = check_root();
    let has_tracefs = check_tracefs();
    let has_perf = check_perf();
    let has_trace_cmd = check_trace_cmd();
    let has_nvme_cli = check_nvme_cli();

    EnvCheckResult {
        kernel_version,
        has_root,
        has_tracefs,
        has_perf,
        has_trace_cmd,
        has_nvme_cli,
    }
}

fn get_kernel_version() -> String {
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to execute uname");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn check_root() -> bool {
    let o = Command::new("id")
        .arg("-u")
        .output()
        .expect("Failed to execute id");
    let uid = String::from_utf8_lossy(&o.stdout).trim().to_string();
    uid == "0"
}

fn check_tracefs() -> bool {
    let o = Command::new("mount")
        .output()
        .expect("Failed to execute mount");
    let mounts = String::from_utf8_lossy(&o.stdout);
    mounts.lines().any(|line| line.contains("tracefs"))
}

fn check_perf() -> bool {
    let o = Command::new("which")
        .arg("perf")
        .output()
        .expect("Failed to execute which");
    o.status.success()
}

fn check_trace_cmd() -> bool {
    let o = Command::new("which")
        .arg("trace-cmd")
        .output()
        .expect("Failed to execute which");
    o.status.success()
}

fn check_nvme_cli() -> bool {
    let o = Command::new("which")
        .arg("nvme")
        .output()
        .expect("Failed to execute which");
    o.status.success()
}
