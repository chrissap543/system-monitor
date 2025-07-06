use std::thread;
use std::time::Duration;

use clap::Parser;
use sysinfo::{Disks, System};

use system_monitor::*;

#[derive(Parser)]
#[command(name = "system-monitor")]
#[command(about = "A simple system monitoring tool")]
struct Args {
    #[arg(short, long, default_value = "2")]
    interval: u64,

    #[arg(short, long)]
    once: bool,

    #[arg(long)]
    no_color: bool,
}

fn main() {
    let args = Args::parse();
    let mut sys = System::new_all();

    if args.once {
        show_stats(&mut sys, !args.no_color);
        return;
    }

    loop {
        show_stats(&mut sys, !args.no_color);
        thread::sleep(Duration::from_secs(args.interval));
    }
}

fn show_stats(sys: &mut System, use_color: bool) {
    sys.refresh_all();

    print!("\x1B[2J");

    println!("=== SYSTEM MONITOR ===");
    println!();

    let cpu_usage = sys.global_cpu_usage();
    println!("CPU Usage: {:.1}%", cpu_usage);
    print_progress_bar(cpu_usage as f64, 100.0, use_color);
    println!();

    let used_memory = sys.used_memory();
    let total_memory = sys.total_memory();
    let memory_percent = (used_memory as f64 / total_memory as f64) * 100.0;
    println!(
        "Memory: {:.1}% ({} / {})",
        memory_percent,
        format_bytes(used_memory),
        format_bytes(total_memory)
    );
    print_progress_bar(memory_percent, 100.0, use_color);

    let used_swap = sys.used_swap();
    let total_swap = sys.total_swap();
    if total_swap > 0 {
        let swap_percent = (used_swap as f64 / total_swap as f64) * 100.0;
        println!(
            "Swap: {:.1}% ({} / {})",
            swap_percent,
            format_bytes(used_swap),
            format_bytes(total_swap)
        );
        print_progress_bar(swap_percent, 100.0, use_color);
    }
    println!();

    println!("Disk Usage:");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let used = disk.total_space() - disk.available_space();
        let total = disk.total_space();
        let percent = (used as f64 / total as f64) * 100.0;

        println!(
            "  {}: {:.1}% ({} / {})",
            disk.mount_point().display(),
            percent,
            format_bytes(used),
            format_bytes(total)
        );
        print_progress_bar(percent, 100.0, use_color);
    }

    println!();
    println!("Press Ctrl+C to exit...");
}

fn print_progress_bar(current: f64, max: f64, use_color: bool) {
    println!("{}", generate_progress_bar(current, max, use_color));
}

