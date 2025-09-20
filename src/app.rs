use crate::fetch::{get_packages, get_outdated_packages, get_recent_packages, get_disk_info};

pub struct App {
    pub packages: Vec<crate::fetch::PackageInfo>,
    pub outdated_packages: Vec<crate::fetch::PackageInfo>,
    pub recent_packages: Vec<crate::fetch::PackageInfo>,
    pub disk_info: Vec<crate::fetch::DiskInfo>,
}

impl App {
    pub fn new() -> Self {
        let packages = get_packages();
        let outdated_packages = get_outdated_packages();
        let recent_packages = get_recent_packages();
        let disk_info = get_disk_info();
        
        Self {
            packages,
            outdated_packages,
            recent_packages,
            disk_info,
        }
    }
    
    pub fn update(&mut self) {
        self.packages = get_packages();
        self.outdated_packages = get_outdated_packages();
        self.recent_packages = get_recent_packages();
        self.disk_info = get_disk_info();
    }
    
    pub fn get_package_stats(&self) -> PackageStats {
        let mut pacman_packages = Vec::new();
        let mut aur_packages = Vec::new();
        let mut apt_packages = Vec::new();
        let mut flatpak_packages = Vec::new();
        
        for package in &self.packages {
            match package.source.as_str() {
                "pacman" => pacman_packages.push(package),
                "aur" => aur_packages.push(package),
                "apt" => apt_packages.push(package),
                "flatpak" => flatpak_packages.push(package),
                _ => {}
            }
        }
        
        PackageStats {
            total: pacman_packages.len() + aur_packages.len() + apt_packages.len() + flatpak_packages.len(),
            pacman: pacman_packages.len(),
            aur: aur_packages.len(),
            apt: apt_packages.len(),
            flatpak: flatpak_packages.len(),
            outdated: self.outdated_packages.len(),
        }
    }
}

#[derive(Debug)]
pub struct PackageStats {
    pub total: usize,
    pub pacman: usize,
    pub aur: usize,
    pub apt: usize,
    pub flatpak: usize,
    pub outdated: usize,
}