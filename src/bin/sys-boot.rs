use std::fs::File;
use std::io::{BufRead, BufReader, Write};

const GRUB_CONFIG_PATH: &str = "/etc/default/grub";

fn main() {
    println!("# ");
    println!("#  ⚙️ Optimizing ignition sequence within existing workspace...");
    println!("# ");

    match optimize_boot_parameters() {
        Ok(_) => {
            println!("🔒 GRUB parameters locked! Boot countdown crushed to 0s for an instant start.");
            println!("⚡ Corporate splash screens bypassed. Clean kernel text mode initialized.");
        }
        Err(e) => {
            eprintln!("💥 Failed to rewrite boot structure: {}", e);
            eprintln!("🤔 Note: Ensure this utility runs with elevated privileges (sudo/root).");
        }
    }
}

fn optimize_boot_parameters() -> std::io::Result<()> {
    let file = File::open(GRUB_CONFIG_PATH)?;
    let reader = BufReader::new(file);
    let mut updated_lines = Vec::new();

    for line in reader.lines() {
        let mut l = line?;
        
        // 1. Wipe out the artificial 5-second menu delay
        if l.starts_with("GRUB_TIMEOUT=") {
            l = String::from("GRUB_TIMEOUT=0");
        }
        // 2. Erase generic quiet/splash screens so the crisp kernel text flows down the monitor
        else if l.starts_with("GRUB_CMDLINE_LINUX_DEFAULT=") {
            l = String::from("GRUB_CMDLINE_LINUX_DEFAULT=\"loglevel=3 systemd.show_status=true\"");
        }
        // 3. Force pure text layout initialization rather than loading heavy graphic frames early
        else if l.starts_with("#GRUB_TERMINAL=console") {
            l = String::from("GRUB_TERMINAL=console");
        }

        updated_lines.push(l);
    }

    // Write back changes to the standard live filesystem block
    let mut file = File::create(GRUB_CONFIG_PATH)?;
    for line in updated_lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
