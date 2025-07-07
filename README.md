# SysMon - Simple System Monitor

A basic Rust system monitor that displays CPU, memory, and disk usage.

## What it does

- Shows CPU usage percentage with progress bar
- Shows memory usage (used/total in MB and percentage) with progress bar  
- Shows swap usage if available (used/total in MB and percentage) with progress bar
- Shows disk usage for all mounted drives (used/total in GB and percentage) with progress bars
- Updates every 2 seconds (or custom interval)
- Clears screen between updates
- Command line options for interval and one-shot mode

## How to run

```bash
# Run with defaults (2 second intervals)
cargo run

# Run once and exit
cargo run -- --once

# Custom interval (5 seconds)
cargo run -- --interval 5

# Without color
cargo run -- --no-color

# Show help
cargo run -- --help
```

```
=== SYSTEM MONITOR ===

CPU Usage: 1.7%
[                                        ]

Memory: 68.6% (21.3 GB / 31.1 GB)
[███████████████████████████             ]

Disk Usage:
  /: 14.5% (55.4 GB / 382.5 GB)
[█████                                   ]

Press Ctrl+C to exit...
```

