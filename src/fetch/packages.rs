use std::process::Command;

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub source: String, // "pacman", "aur", "apt", "flatpak", "dnf", "rpm"
}

pub fn detect_system() -> Vec<(&'static str, Vec<&'static str>)> {
    if Command::new("pacman").arg("--version").output().is_ok() {
        return vec![
            ("pacman", vec!["-Q"]),
            ("yay", vec!["-Qm"]), // AUR packages
        ];
    }
    
    if Command::new("apt").arg("--version").output().is_ok() {
        return vec![
            ("dpkg", vec!["-l"]),
            ("flatpak", vec!["list"]), // Flatpak packages
        ];
    }
    
    if Command::new("dnf").arg("--version").output().is_ok() {
        return vec![
            ("rpm", vec!["-qa"]), // All installed packages
            ("flatpak", vec!["list"]), // Flatpak packages
        ];
    }
    
    vec![]
}

pub fn get_packages() -> Vec<PackageInfo> {
    let mut packages = Vec::new();
    let commands = detect_system();
    
    for (command, args) in commands {
        let output = Command::new(command)
            .args(&args)
            .output();
            
        match output {
            Ok(result) if result.status.success() => {
                let output_str = String::from_utf8_lossy(&result.stdout);
                let source = match command {
                    "pacman" => "pacman",
                    "yay" => "aur", 
                    "dpkg" => "apt",
                    "rpm" => "dnf",
                    "flatpak" => "flatpak",
                    _ => "unknown"
                };
                
                for line in output_str.lines() {
                    if !line.trim().is_empty() {
                        packages.push(PackageInfo {
                            name: line.to_string(),
                            version: "".to_string(), // TODO: parse version
                            source: source.to_string(),
                        });
                    }
                }
            }
            Ok(result) => {
                eprintln!("Error executing {}: {}", command, String::from_utf8_lossy(&result.stderr));
            }
            Err(_) => {
                // Command not found - this is normal
            }
        }
    }
    
    packages
}

pub fn get_outdated_packages() -> Vec<PackageInfo> {
    let mut outdated = Vec::new();
    
    if Command::new("pacman").arg("--version").output().is_ok() {
        // Sync database for more accurate results
        let _sync_output = Command::new("pacman")
            .arg("-Sy")  // database synchronization
            .output();
        
        let output = Command::new("pacman")
            .arg("-Qu")  // flag for outdated packages
            .output();
            
        match output {
            Ok(result) if result.status.success() => {
                let output_str = String::from_utf8_lossy(&result.stdout);
                for line in output_str.lines() {
                    if !line.trim().is_empty() {
                        outdated.push(PackageInfo {
                            name: line.to_string(),
                            version: "".to_string(),
                            source: "pacman".to_string(),
                        });
                    }
                }
            }
            Ok(_) => {
                // No outdated packages - this is normal
            }
            Err(_) => {
                // Command not found
            }
        }
    }
    
    // Check for yay for AUR packages
    if Command::new("yay").arg("--version").output().is_ok() {
        let output = Command::new("yay")
            .arg("-Qu")  // flag for outdated AUR packages
            .output();
            
        match output {
            Ok(result) if result.status.success() => {
                let output_str = String::from_utf8_lossy(&result.stdout);
                for line in output_str.lines() {
                    if !line.trim().is_empty() {
                        outdated.push(PackageInfo {
                            name: line.to_string(),
                            version: "".to_string(),
                            source: "aur".to_string(),
                        });
                    }
                }
            }
            Ok(_) => {
                // No outdated packages
            }
            Err(_) => {
                // Command not found
            }
        }
    }
    
    // Check for dnf for Fedora packages
    if Command::new("dnf").arg("--version").output().is_ok() {
        let output = Command::new("dnf")
            .arg("list")
            .arg("upgrades")
            .output();
            
        match output {
            Ok(result) if result.status.success() => {
                let output_str = String::from_utf8_lossy(&result.stdout);
                for line in output_str.lines().skip(1) { // Skip header
                    if !line.trim().is_empty() && !line.starts_with("Last metadata") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            outdated.push(PackageInfo {
                                name: parts[0].to_string(),
                                version: parts[1].to_string(),
                                source: "dnf".to_string(),
                            });
                        }
                    }
                }
            }
            Ok(_) => {
                // No outdated packages
            }
            Err(_) => {
                // Command not found
            }
        }
    }
    
    outdated
}

pub fn get_recent_packages() -> Vec<PackageInfo> {
    let mut recent = Vec::new();
    
    // Get recently installed packages via pacman
    if Command::new("pacman").arg("--version").output().is_ok() {
        let output = Command::new("pacman")
            .arg("-Q")  // all installed packages
            .output();
            
        match output {
            Ok(result) if result.status.success() => {
                let output_str = String::from_utf8_lossy(&result.stdout);
                let mut lines: Vec<&str> = output_str.lines().collect();
                
                // Take last 5 packages
                lines.reverse();
                for line in lines.iter().take(5) {
                    if !line.trim().is_empty() {
                        recent.push(PackageInfo {
                            name: line.to_string(),
                            version: "".to_string(),
                            source: "pacman".to_string(),
                        });
                    }
                }
            }
            Ok(_) => {
                // Command executed but no output
            }
            Err(_) => {
                // Command not found
            }
        }
    }
    
    // Get recently installed packages via dnf for Fedora
    if Command::new("dnf").arg("--version").output().is_ok() {
        let output = Command::new("dnf")
            .arg("history")
            .arg("list")
            .arg("installed")
            .arg("--limit=5")
            .output();
            
        match output {
            Ok(result) if result.status.success() => {
                let output_str = String::from_utf8_lossy(&result.stdout);
                for line in output_str.lines().skip(1) { // Skip header
                    if !line.trim().is_empty() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 3 {
                            // Extract package name from the command
                            let command_part = parts[2..].join(" ");
                            if command_part.contains("install") {
                                let package_name = command_part
                                    .split_whitespace()
                                    .find(|s| !s.starts_with("install") && !s.starts_with("-"))
                                    .unwrap_or("unknown");
                                
                                recent.push(PackageInfo {
                                    name: package_name.to_string(),
                                    version: "".to_string(),
                                    source: "dnf".to_string(),
                                });
                            }
                        }
                    }
                }
            }
            Ok(_) => {
                // Command executed but no output
            }
            Err(_) => {
                // Command not found
            }
        }
    }
    
    recent
}
