mod cli;
mod env_check;

use analyzers::analyze_all;
use clap::Parser;
use collectors::collect_all;
use report::render_all;
use std::fmt::Display;

pub enum ProfileType {
    Quick,
    Default,
    Deep,
}

impl Display for ProfileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileType::Quick => write!(f, "quick"),
            ProfileType::Default => write!(f, "default"),
            ProfileType::Deep => write!(f, "deep"),
        }
    }
}

/// command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// block device, e.g. nvme0n1
    #[arg(long)]
    device: String,
    /// Process ID
    #[arg(long, default_value_t = 1)]
    pid: u32,
    /// Duration in seconds
    #[arg(short, long, default_value_t = 10)]
    duration: u32,
    /// profile <quick|default|deep>
    #[arg(short, long, default_value = "default")]
    profile: String,
    /// format <text|json|markdown>
    #[arg(short, long, default_value = "text")]
    format: String,
    /// Output file path
    #[arg(short, long, default_value = "report.txt")]
    output: String,
    /// no perf
    #[arg(long, default_value_t = false)]
    no_perf: bool,
    /// no trace
    #[arg(long, default_value_t = false)]
    no_trace: bool,
    /// no nvme-cli
    #[arg(long, default_value_t = false)]
    no_nvme_cli: bool,
}
pub struct Profile {
    pub name: ProfileType,
    pub duration_s: u32,
}

pub struct InspectRequest {
    pub device: Option<String>,
    pub pid: Option<u32>,
    pub duration_s: u32,
    pub profile: Profile,
}

fn run_inspect(request: InspectRequest) {
    //device check
    let o = std::process::Command::new("lsblk")
        .arg("-o")
        .arg("NAME")
        .output()
        .expect("Failed to execute lsblk");
    let devices = String::from_utf8_lossy(&o.stdout);
    let binding = "".to_string();

    println!("Available devices:\n{}", devices);

    let device_name = request.device.as_ref().unwrap_or(&binding);
    if !devices.lines().any(|line| line.contains(device_name)) {
        eprintln!("Error: Device {} not found", device_name);
        return;
    }

    // pid check
    let o = std::process::Command::new("ps")
        .arg("-e")
        .arg("-o")
        .arg("pid")
        .output()
        .expect("Failed to execute ps");
    let pids = String::from_utf8_lossy(&o.stdout);
    let pid_str = request.pid.map(|p| p.to_string()).unwrap_or_default();
    if !pids.lines().any(|line| line.trim() == pid_str) {
        eprintln!("Error: PID {} not found", pid_str);
        return;
    }
    println!("Running inspect with profile: {}", request.profile.name);
}

fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.pid);

    let r = InspectRequest {
        device: Some(args.device),
        pid: Some(args.pid as u32),
        duration_s: args.duration,
        profile: Profile {
            name: match args.profile.as_str() {
                "quick" => ProfileType::Quick,
                "deep" => ProfileType::Deep,
                _ => ProfileType::Default,
            },
            duration_s: args.duration as u32,
        },
    };

    run_inspect(r);

    let _ = cli::run();

    let env_check_result = env_check::check();

    println!(
        "Environment Check: kernel_version={}, has_root={}, has_tracefs={}, has_perf={}, has_trace_cmd={}, has_nvme_cli={}",
        env_check_result.kernel_version,
        env_check_result.has_root,
        env_check_result.has_tracefs,
        env_check_result.has_perf,
        env_check_result.has_trace_cmd,
        env_check_result.has_nvme_cli
    );

    let collects = collect_all();
    let analyses = analyze_all();
    let reports = render_all();

    println!("Collected: {:?}", collects);
    println!("Analyzed: {:?}", analyses);
    println!("Reports: {:?}", reports);
}
