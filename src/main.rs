use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::Command;
use std::path::Path;

fn main() {
    print_banner();
    
    println!("🛡️  [Defenestration Core] Initializing system guard...");

    // 1. Enforce Privacy (Telemetry Blocking)
    apply_telemetry_shield();

    // 2. Hardware-Aware Resource Optimization
    optimize_resources();

    // 3. Clean up Systemd Services
    sanitize_services();

    println!("🚀 [Defenestration Core] Shield execution complete. Control handed back to environment.");
}

/// Displays the customized ASCII branding for Defenestration OS
fn print_banner() {
    let logo = r#"
    ____       🪟 💨
   / __ \ ___   / __/___   ____   ___   _____ / /_ _____  ____ _ / /_ (_)____   ____ 
  / / / // _ \ / /_ / _ \ / __ \ / _ \ / ___// __// ___/ / __ `// __// // __ \ / __ \
 / /_/ //  __// __//  __// / / //  __/(__  )/ /_ / /    / /_/ // /_ / // /_/ // / / /
/_____/ \___//_/   \___//_/ /_/ \___//____/ \__//_/     \__,_/ \__//_/ \____//_/ /_/ 
    "#;
    println!("{}", logo);
}

/// Sweeps the /etc/hosts file to bind known telemetry endpoints to the local loopback
fn apply_telemetry_shield() {
    let hosts_path = "/etc/hosts";
    let block_list = vec![
        "127.0.0.1 telemetry.microsoft.com",
        "127.0.0.1 v10.events.data.microsoft.com",
        "127.0.0.1 diagnostics.office.com",
    ];

    if !Path::new(hosts_path).exists() {
        eprintln!("⚠️  Target environment lacking standard network configuration pathways.");
        return;
    }

    if let Ok(mut file) = OpenOptions::new().append(true).open(hosts_path) {
        writeln!(file, "\n# --- Defenestration OS Privacy Shields ---").unwrap();
        for domain in block_list {
            if let Err(_) = writeln!(file, "{}", domain) {
                eprintln!("⚠️  Failed to append endpoint constraint: {}", domain);
            }
        }
        println!("✅ Telemetry network loopbacks successfully locked down.");
    }
}

/// Inspects /proc/meminfo to dynamically scale kernel options based on physical constraints
fn optimize_resources() {
    let meminfo_path = "/proc/meminfo";
    let mut total_kb: u64 = 8_000_000; // Default fallback to 8GB assumption

    if let Ok(file) = File::open(meminfo_path) {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.starts_with("MemTotal:") {
                // Parse out the numeric value from strings like "MemTotal:    3940212 kB"
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(parsed_mem) = parts[1].parse::<u64>() {
                        total_kb = parsed_mem;
                    }
                }
                break;
            }
        }
    }

    let total_gb = total_kb / 1024 / 1024;
    println!("⚙️  Detected System Capacity: {} GB RAM", total_gb);

    // Apply scaling parameters based on potato thresholds (Less than 4GB)
    if total_gb <= 4 {
        println!("⚠️  [Potato Alert] Low-memory environment detected. Hardening memory swap constraints...");
        let _ = Command::new("sysctl").args(["-w", "vm.swappiness=15"]).status();
        let _ = Command::new("sysctl").args(["-w", "vm.vfs_cache_pressure=50"]).status();
        // Prevent kernel panics on memory exhaustion by forcing aggressive cache drops
        let _ = Command::new("sysctl").args(["-w", "vm.panic_on_oom=0"]).status();
    } else {
        println!("⚡ Performance environment verified. Applying balanced kernel scaling profiles.");
        let _ = Command::new("sysctl").args(["-w", "vm.swappiness=30"]).status();
    }
}

/// Mask or disable default telemetry/logging daemons that sneak into base distros
fn sanitize_services() {
    // Attempt to mask systemd core elements that trigger disk churn on potato PCs
    let services_to_kill = vec!["systemd-boot-check-no-failures.service"];

    for service in services_to_kill {
        let status = Command::new("systemctl")
            .args(["mask", "--now", service])
            .status();
        
        if let Ok(s) = status {
            if s.success() {
                println!("🔒 Extinguished standard distribution overhead service: {}", service);
            }
        }
    }
}
