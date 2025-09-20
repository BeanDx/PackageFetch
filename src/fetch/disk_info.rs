use std::process::Command;

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub device: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percentage: f64,
}

pub fn get_disk_info() -> Vec<DiskInfo> {
    let mut disks = Vec::new();
    
    // Get disk usage using df command
    let output = Command::new("df")
        .arg("-h")
        .arg("--output=source,target,size,used,avail,pcent")
        .output();
    
    match output {
        Ok(result) if result.status.success() => {
            let output_str = String::from_utf8_lossy(&result.stdout);
            let lines: Vec<&str> = output_str.lines().collect();
            
            // Skip header line
            for line in lines.iter().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    let device = parts[0].to_string();
                    let mount_point = parts[1].to_string();
                    
                    // Skip special filesystems
                    if device.starts_with("/dev/") && !device.contains("loop") {
                        if let (Ok(total), Ok(used), Ok(available)) = (
                            parse_size(parts[2]),
                            parse_size(parts[3]),
                            parse_size(parts[4])
                        ) {
                            let usage_percentage = if total > 0 {
                                (used as f64 / total as f64) * 100.0
                            } else {
                                0.0
                            };
                            
                            disks.push(DiskInfo {
                                device,
                                mount_point,
                                total,
                                used,
                                available,
                                usage_percentage,
                            });
                        }
                    }
                }
            }
        }
        Ok(_) => {
            // df command executed but no output
        }
        Err(_) => {
            // df command not found or failed
        }
    }
    
    // Sort by usage percentage (highest first)
    disks.sort_by(|a, b| b.usage_percentage.partial_cmp(&a.usage_percentage).unwrap());
    disks
}

fn parse_size(size_str: &str) -> Result<u64, std::num::ParseIntError> {
    // Parse sizes like "100G", "500M", "1T"
    let size_str = size_str.trim();
    let (number, unit) = if size_str.ends_with('G') {
        (size_str.trim_end_matches('G'), 1024 * 1024 * 1024)
    } else if size_str.ends_with('M') {
        (size_str.trim_end_matches('M'), 1024 * 1024)
    } else if size_str.ends_with('K') {
        (size_str.trim_end_matches('K'), 1024)
    } else if size_str.ends_with('T') {
        (size_str.trim_end_matches('T'), 1024 * 1024 * 1024 * 1024)
    } else {
        (size_str, 1)
    };
    
    number.parse::<u64>().map(|n| n * unit)
}

pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "K", "M", "G", "T"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{:.0}{}", size, UNITS[unit_index])
    } else {
        format!("{:.1}{}", size, UNITS[unit_index])
    }
}