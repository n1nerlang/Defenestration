use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

// Hardcoded telemetry sinkholes to drop straight into the local routing loop
const BLOCKLIST: &[&str] = &[
    "telemetry.microsoft.com",
    "vortex.data.microsoft.com",
    "settings-win.data.microsoft.com",
    "watson.telemetry.microsoft.com",
    "diagnostics.office.com",
];

fn main() {
    println!("⚡ SHIELD DAEMON: Initializing Defenestration Core Security Layer...");

    // 1. Run the boot-time telemetry sync block
    if let Err(e) = enforce_telemetry_sinkhole() {
        eprintln!("❌ SHIELD DAEMON: Failed to lock hosts structure: {}", e);
    } else {
        println!("🔒 SHIELD DAEMON: Telemetry blocklist successfully bound to /etc/hosts.");
    }

    // 2. Fire up the high-precision memory monitoring loop
    println!("🥔 SHIELD DAEMON: Memory sentinel active. Monitoring hardware state...");
    loop {
        match check_memory_pressure() {
            Ok(available_pct) => {
                if available_pct < 15.0 {
                    println!(
                        "⚠️ SHIELD DAEMON: CRITICAL RAM WARNING ({:.1}% left). Executing memory salvage protocols...",
                        available_pct
                    );
                    salvage_memory();
                }
            }
            Err(e) => eprintln!("❌ SHIELD DAEMON: Critical log read failure: {}", e),
        }

        // Sleep for 3 seconds between system checks to maintain zero CPU overhead
        thread::sleep(Duration::from_secs(3));
    }
}

/// Parses /proc/meminfo directly to compute precise real-time available memory percentages
fn check_memory_pressure() -> std::io::Result<f32> {
    let file = File::open("/proc/meminfo")?;
    let reader = BufReader::new(file);

    let mut mem_total = 0.0;
    let mut mem_available = 0.0;

    for line in reader.lines() {
        let l = line?;
        if l.starts_with("MemTotal:") {
            mem_total = parse_mem_kb(&l);
        } else if l.starts_with("MemAvailable:") {
            mem_available = parse_mem_kb(&l);
            break; // Everything we need is fetched, break out early
        }
    }

    if mem_total > 0.0 {
        Ok((mem_available / mem_total) * 100.0)
    } else {
        Ok(100.0)
    }
}

/// Helper function to extract numerical values from /proc/meminfo raw strings
fn parse_mem_kb(line: &str) -> f32 {
    line.split_whitespace()
        .nth(1)
        .and_then(|val| val.parse::<f32>().ok())
        .unwrap_or(0.0)
}

/// Forces the Linux kernel to clear pagecaches, dentries, and inodes from memory layers
fn salvage_memory() {
    // Equivalent to running: sync && echo 3 > /proc/sys/vm/drop_caches
    // We drop caches to give low-RAM potato PCs breathing room right before an out-of-memory crash
    let _ = std::process::Command::new("sync").status();
    
    if let Ok(mut file) = OpenOptions::new().write(true).open("/proc/sys/vm/drop_caches") {
        if let Err(e) = file.write_all(b"3\n") {
            eprintln!("❌ SHIELD DAEMON: Cache drop execution denied: {}", e);
        } else {
            println!("♻️ SHIELD DAEMON: Purged inactive kernel caches successfully.");
        }
    }
}

/// Inspects /etc/hosts and writes our custom blocklist records if they do not already exist
fn enforce_telemetry_sinkhole() -> std::io::Result<()> {
    let hosts_path = "/etc/hosts";
    
    // Read the current hosts setup to avoid writing redundant duplicates
    let file = File::open(hosts_path)?;
    let reader = BufReader::new(file);
    let mut current_contents = String::new();
    for line in reader.lines() {
        current_contents.push_str(&line?);
        current_contents.push('\n');
    }

    // Open file in append mode to append new blocks cleanly at the bottom
    let mut file = OpenOptions::new().append(true).open(hosts_path)?;
    let mut written = false;

    for domain in BLOCKLIST {
        let entry = format!("127.0.0.1 {}\n", domain);
        if !current_contents.contains(domain) {
            file.write_all(entry.as_bytes())?;
            println!("   + Added sinkhole node: {}", domain);
            written = true;
        }
    }

    if written {
        file.write_all(b"\n# Defenestration OS Telemetry Guard Terminal Block\n")?;
    }
    Ok(())
}
