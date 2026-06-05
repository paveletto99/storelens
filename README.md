# StorLens

StorLens is a Linux storage diagnostics tool written in Rust.

It collects information from the storage stack, analyzes the results, and generates actionable findings to help engineers investigate performance issues.

The project is designed for NVMe, SSD, and general block-device workloads and can be used to diagnose latency, queueing, saturation, scheduling, and configuration problems.

## Features

- Storage environment inspection
- NVMe device health checks
- Block device configuration analysis
- I/O performance collection
- Scheduler and CPU locality insights
- Automated findings and recommendations
- Human-readable and JSON reports

## Goals

StorLens aims to answer questions such as:

- Is the storage device saturated?
- Where is latency introduced?
- Is queue depth appropriate?
- Are scheduler delays affecting I/O?
- Is the device healthy?
- Are there configuration issues impacting performance?

## Architecture

```text
CLI
  ↓
Collectors
  ↓
Normalizers
  ↓
Analyzers
  ↓
Reports
```

### Collectors

StorLens gathers information from:

* `/sys`
* `/proc`
* `iostat`
* `perf`
* `trace-cmd`
* `nvme-cli`

Additional collectors such as eBPF may be added in future releases.

## Installation

### Build from source

```bash
git clone https://github.com/your-org/storlens.git
cd storlens
cargo build --release
```

Binary:

```bash
./target/release/storlens
```

## Usage

Inspect a device:

```bash
storlens inspect --device nvme0n1
```

Run a timed inspection:

```bash
storlens inspect --device nvme0n1 --duration 15
```

Inspect a workload:

```bash
storlens inspect --pid 1234
```

Generate JSON output:

```bash
storlens inspect --device nvme0n1 --format json
```

## Example Output

```text
Summary
-------
Device: nvme0n1
Duration: 15s

Findings
--------
[WARNING] Device saturation likely
[WARNING] Scheduler delays detected
[INFO] NVMe health is normal

Key Metrics
-----------
Average latency: 3.8 ms
P99 latency: 5.1 ms
Queue depth: 12
Utilization: 93%
```

## Requirements

Recommended tools:

* perf
* trace-cmd
* sysstat (iostat)
* nvme-cli

Some features may require elevated privileges.

## Roadmap

### v0.1

* Environment checks
* Sysfs and procfs collectors
* iostat integration
* NVMe health inspection
* Basic findings engine

### v0.2

* perf integration
* Trace collection
* Markdown and JSON reports

### v0.3

* eBPF collectors
* Latency histograms
* Queue-depth analysis
* Continuous monitoring mode

## License

MIT
