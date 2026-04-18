# io-culprit

Lightweight Rust daemon + offline analyzer for tracking down disk I/O culprits on Debian VMs.

When your VM locks up from sustained high disk I/O after running for weeks, `io-culprit` captures evidence in real time and tells you who did it after reboot.

## How it works

**`io-watchdog`** runs as a systemd service, sampling `/proc/diskstats` and `/proc/stat` every 15 seconds with near-zero overhead. When it detects anomalous I/O patterns, it snapshots `pidstat`, `ps`, `vmstat`, `/proc/<pid>/io`, and kernel logs into an incident directory.

**`io-report`** runs manually after reboot. It reads the incident snapshots, cross-references `atop` history and kernel logs, scores each process, and outputs a ranked suspect list with evidence.

## Quick start

### Option 1: Download pre-built binaries (recommended)

Go to the [Releases page](https://github.com/nick3/io-culprit/releases) and download the archive matching your architecture:

| Architecture | File |
|---|---|
| x86_64 (Intel/AMD) | `io-culprit-x86_64-linux-gnu.tar.gz` |
| aarch64 (ARM64) | `io-culprit-aarch64-linux-gnu.tar.gz` |
| x86_64 static (musl) | `io-culprit-x86_64-linux-musl.tar.gz` |
| aarch64 static (musl) | `io-culprit-aarch64-linux-musl.tar.gz` |

Then install on your Debian VM:

```bash
# Download (replace URL with the actual release link)
wget https://github.com/nick3/io-culprit/releases/latest/download/io-culprit-x86_64-linux-gnu.tar.gz

# Extract
tar xzf io-culprit-x86_64-linux-gnu.tar.gz

# Install binaries
sudo cp io-watchdog io-report /usr/local/bin/

# Install config
sudo mkdir -p /etc/io-culprit
sudo cp config.yaml /etc/io-culprit/

# Install and start service
sudo cp io-watchdog.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now io-watchdog

# Verify it's running
sudo systemctl status io-watchdog
```

### Option 2: Build from source

```bash
# Build
cargo build --release

# Install binaries
sudo cp target/release/io-watchdog /usr/local/bin/
sudo cp target/release/io-report /usr/local/bin/

# Install config
sudo mkdir -p /etc/io-culprit
sudo cp config/config.yaml /etc/io-culprit/

# Install and start service
sudo cp systemd/io-watchdog.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now io-watchdog
```

## After an incident

When the VM locks up from high disk I/O and you force-reboot it:

**Step 1: Run the report**

```bash
sudo io-report
```

It automatically reads event snapshots from `/var/log/io-culprit/`, cross-references `atop` history and the previous boot's kernel logs, and outputs a ranked suspect list.

**Step 2: Read the results**

Look at the Top 1-3 suspects and their evidence. The JSON report is saved at:

```
/var/log/io-culprit/incident-*/report.json
```

**Step 3: If the report shows "no incident data"**

This means the system froze too fast for the watchdog to trigger. Fall back to manual investigation:

```bash
# Check atop history (navigate to the time of the incident)
sudo atop -r /var/log/atop/atop_YYYYMMDD

# Check kernel logs from the previous boot
journalctl -k -b -1

# Check for disk/filesystem errors
dmesg -T | grep -iE 'blk|ext4|xfs|i/o error|reset|timeout'
```

**Output example:**

```
Incident: incident-20260418-021130
Device: sda
Time Range: 2026-04-18T02:11:30Z..2026-04-18T02:18:45Z
Summary: device saturated, userspace process dominant
Suspect #1: postgres [userspace-process] score=20 evidence=continuous top writer; %util 99%; await 420ms
Next: inspect postgres logs and query patterns
```

## Configuration

Edit `/etc/io-culprit/config.yaml`:

```yaml
interval_secs: 15          # sampling interval
util_threshold: 90          # %util trigger threshold
await_threshold_ms: 50      # await trigger threshold (ms)
util_critical: 98           # single-sample critical threshold
iowait_threshold: 25        # system iowait trigger
consecutive_triggers: 2     # consecutive samples before triggering
max_snapshot_rounds: 3      # max snapshots per incident
incident_dir: /var/log/io-culprit
retention_days: 30
```

## Trigger rules

An incident is triggered when any of these conditions are met:

1. Same device has `%util >= 90` and `await >= 50ms` for 2 consecutive samples
2. Any device hits `%util >= 98` in a single sample
3. System `iowait >= 25%` with any device `%util >= 80%` for 2 consecutive samples

## Suspect classification

Each suspect is classified into one of:

- `userspace-process` — regular application (e.g., postgres, rsync)
- `kernel-thread` — kernel worker (e.g., kworker)
- `filesystem-writeback` — journal/flush threads (e.g., jbd2, flush-*)
- `memory-pressure` — swap-related (e.g., kswapd)
- `host-or-storage-suspect` — no clear guest-side culprit found

## Prerequisites

- Debian (or any Linux with `/proc/diskstats` and systemd)
- `atop` and `sysstat` installed for richer evidence:

```bash
sudo apt install -y atop sysstat
sudo systemctl enable --now atop sysstat
```

## Building from source

```bash
cargo build --release
cargo test
```

## License

MIT
