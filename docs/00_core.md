# StorLens Architecture and Design

## Overview

StorLens is a Linux storage diagnostics and analysis tool written in Rust.

Its primary objective is to automate storage performance investigations by collecting evidence from multiple layers of the Linux I/O stack, correlating the results, and producing actionable findings.

StorLens focuses on:

* NVMe devices
* SSD storage
* Block devices
* Filesystems
* Storage performance troubleshooting
* Latency analysis
* Queueing analysis
* Scheduler impact on I/O

StorLens is not intended to be a benchmark tool. Instead, it acts as an investigation assistant that helps performance engineers identify bottlenecks and misconfigurations.

---

# Design Principles

## Evidence First

All conclusions must be based on collected evidence.

Every finding should include:

* What was observed
* Why it matters
* Supporting metrics
* Suggested next steps

Example:

```text
Finding:
Scheduler delay contributes to p99 latency

Evidence:
- Average device service time: 450 μs
- Completion latency: 4.3 ms
- CPU migrations: 750/s

Recommendation:
Investigate CPU affinity and IRQ placement.
```

---

## Layered Analysis

Storage performance issues rarely originate from a single layer.

StorLens analyzes:

```text
Application
    ↓
Userspace I/O
    ↓
Kernel I/O Path
    ↓
Block Layer
    ↓
Device Driver
    ↓
NVMe Device
```

The tool attempts to identify where latency is introduced.

---

## Tool Agnostic

StorLens does not depend on a single observability technology.

Collectors can use:

* procfs
* sysfs
* perf
* trace-cmd
* nvme-cli
* eBPF

The analysis engine works on normalized data structures.

---

## Extensible

New collectors should be added without changing analyzers.

New analyzers should be added without changing collectors.

---

# System Architecture

```text
┌────────────┐
│ CLI        │
└─────┬──────┘
      │
      ▼
┌────────────┐
│ Planner    │
└─────┬──────┘
      │
      ▼
┌────────────┐
│ Collectors │
└─────┬──────┘
      │
      ▼
┌────────────┐
│ Parsers    │
└─────┬──────┘
      │
      ▼
┌────────────┐
│ Analyzer   │
└─────┬──────┘
      │
      ▼
┌────────────┐
│ Reporter   │
└────────────┘
```

---

# Inspection Workflow

## Step 1: Capability Discovery

Before collecting data, StorLens discovers available capabilities.

Checks include:

* Kernel version
* Root privileges
* TraceFS availability
* perf availability
* trace-cmd availability
* nvme-cli availability
* Available tracepoints
* Available devices

Example:

```text
Capabilities
------------
TraceFS: Available
perf: Available
trace-cmd: Available
nvme-cli: Available
Root: Yes
```

---

## Step 2: Execution Plan

StorLens creates a collection plan based on:

* Requested profile
* Available tools
* Available permissions

Example:

```text
Profile: default

Collectors:
✓ sysfs
✓ procfs
✓ iostat
✓ nvme-cli
✓ perf stat
✓ trace-cmd
```

---

## Step 3: Data Collection

Collectors gather raw evidence.

Raw evidence is preserved whenever possible.

Example:

```text
artifacts/
├── iostat.txt
├── perf.txt
├── trace.dat
├── nvme-smart.json
└── metadata.json
```

---

## Step 4: Normalization

Collector outputs are converted into internal structures.

Example:

```rust
pub struct DeviceLatency {
    pub average_us: u64,
    pub p50_us: u64,
    pub p95_us: u64,
    pub p99_us: u64,
}
```

This allows analyzers to remain independent from specific tools.

---

## Step 5: Analysis

Analyzers evaluate the normalized data.

Each analyzer is responsible for a specific domain.

Examples:

* Saturation Analyzer
* Queue Analyzer
* Scheduler Analyzer
* Device Health Analyzer
* Configuration Analyzer

---

## Step 6: Reporting

Results are rendered into:

* Terminal output
* JSON
* Markdown

Future:

* HTML
* Prometheus
* OpenTelemetry

---

# Profiles

## Quick

Low overhead inspection.

Data sources:

* sysfs
* procfs
* iostat
* nvme-cli

Use cases:

* Production systems
* Initial triage

---

## Default

Adds runtime diagnostics.

Data sources:

* Quick profile
* perf stat
* tracepoints

Use cases:

* Performance investigations
* Reproducible workloads

---

## Deep

Extended tracing and capture.

Data sources:

* Default profile
* perf record
* Extended trace capture

Use cases:

* Root cause analysis
* Engineering investigations

---

# Collectors

## Sysfs Collector

Purpose:

Read block device configuration.

Data:

```text
/sys/block/<device>/queue/*
```

Examples:

* scheduler
* nr_requests
* rq_affinity
* rotational
* read_ahead_kb
* nomerges

---

## Procfs Collector

Purpose:

Collect kernel runtime statistics.

Sources:

```text
/proc/diskstats
/proc/interrupts
/proc/mounts
/proc/cpuinfo
```

---

## Iostat Collector

Purpose:

Measure runtime storage behavior.

Metrics:

* r/s
* w/s
* rkB/s
* wkB/s
* await
* aqu-sz
* util

---

## NVMe Collector

Purpose:

Collect controller health and metadata.

Commands:

```bash
nvme smart-log
nvme id-ctrl
nvme id-ns
```

Metrics:

* temperature
* percentage used
* media errors
* data integrity errors
* firmware version

---

## Perf Collector

Purpose:

Collect CPU and scheduling metrics.

Metrics:

* task-clock
* context-switches
* cpu-migrations
* page-faults
* cycles

---

## Trace Collector

Purpose:

Capture storage-related tracepoints.

Examples:

```text
block:block_rq_insert
block:block_rq_issue
block:block_rq_complete
sched:sched_switch
sched:sched_wakeup
```

Future:

```text
io_uring:*
nvme:*
workqueue:*
```

---

# Internal Data Model

## Inspection Request

```rust
pub struct InspectRequest {
    pub device: Option<String>,
    pub pid: Option<u32>,
    pub duration_s: u32,
    pub profile: Profile,
}
```

---

## Evidence Bundle

```rust
pub struct EvidenceBundle {
    pub sysfs: Option<SysfsSnapshot>,
    pub procfs: Option<ProcfsSnapshot>,
    pub iostat: Option<IostatSnapshot>,
    pub nvme: Option<NvmeSnapshot>,
    pub perf: Option<PerfSnapshot>,
    pub trace: Option<TraceSnapshot>,
}
```

---

## Findings

```rust
pub struct Finding {
    pub severity: Severity,
    pub title: String,
    pub summary: String,
    pub evidence: Vec<String>,
    pub recommendations: Vec<String>,
}
```

---

# Analyzer Design

## Saturation Analyzer

Determines whether the storage subsystem is overloaded.

Signals:

* High utilization
* Large queue depth
* Rising latency

Finding:

```text
Device saturation likely
```

---

## Queue Analyzer

Measures software-side queueing.

Signals:

* Long insert-to-issue latency
* Queue growth

Finding:

```text
Software queueing dominates latency
```

---

## Scheduler Analyzer

Evaluates CPU scheduling effects.

Signals:

* Wakeup delays
* CPU migrations
* Context switches

Finding:

```text
Scheduler interference detected
```

---

## Device Health Analyzer

Checks NVMe health.

Signals:

* Media errors
* High temperature
* SMART warnings

Finding:

```text
Device health issue detected
```

---

## Configuration Analyzer

Evaluates system tuning.

Signals:

* Queue settings
* Scheduler configuration
* IRQ affinity

Finding:

```text
Potential configuration bottleneck
```

---

# Artifact Management

Every run can optionally store artifacts.

Structure:

```text
run-2026-06-05/
├── report.json
├── report.md
├── iostat.txt
├── perf.txt
├── trace.dat
└── metadata.json
```

Artifacts enable later review and sharing.

---

# Future Roadmap

## eBPF Integration

Goals:

* Low-overhead latency histograms
* Queue depth distributions
* Completion analysis
* Long-running monitoring

Examples:

```text
submit → issue
issue → complete
complete → userspace
```

---

## Continuous Monitoring

Future mode:

```bash
storlens monitor --device nvme0n1
```

Features:

* Periodic sampling
* Live findings
* Alert generation

---

## Prometheus Export

Future metrics:

```text
storlens_latency_p99
storlens_queue_depth
storlens_device_utilization
storlens_nvme_temperature
```

---

## OpenTelemetry

Future integration with observability platforms.

Supported exports:

* OTLP
* Jaeger
* Tempo

---

# Non-Goals

StorLens does not aim to:

* Replace fio
* Replace perf
* Replace trace-cmd
* Replace bpftrace
* Replace vendor monitoring tools

StorLens orchestrates and correlates these tools to accelerate investigations.

---

# Success Criteria

StorLens is successful when a performance engineer can run:

```bash
storlens inspect --device nvme0n1
```

and receive:

1. Relevant evidence
2. Actionable findings
3. Recommended next steps

without manually executing dozens of commands or interpreting raw trace data.
