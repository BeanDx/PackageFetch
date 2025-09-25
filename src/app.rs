use crate::fetch::{get_disk_info, get_outdated_packages, get_packages, get_recent_packages};

pub struct App {
    pub packages: Vec<crate::fetch::PackageInfo>,
    pub outdated_packages: Vec<crate::fetch::PackageInfo>,
    pub recent_packages: Vec<crate::fetch::PackageInfo>,
    pub disk_info: Vec<crate::fetch::DiskInfo>,
    pub error_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let packages = get_packages();

        let (outdated_packages, error_message) = match get_outdated_packages() {
            Ok(v) => (v, None),
            Err(e) => (Vec::new(), Some(e)),
        };

        let recent_packages = get_recent_packages();
        let disk_info = get_disk_info();

        Self {
            packages,
            outdated_packages,
            recent_packages,
            disk_info,
            error_message,
        }
    }

    pub fn update(&mut self) {
        self.packages = get_packages();

        match get_outdated_packages() {
            Ok(v) => {
                self.outdated_packages = v;
                self.error_message = None;
            }
            Err(e) => {
                self.outdated_packages.clear();
                self.error_message = Some(e);
            }
        }

        self.recent_packages = get_recent_packages();
        self.disk_info = get_disk_info();
    }

    pub fn get_package_stats(&self) -> PackageStats {
        let mut pacman_packages = Vec::new();
        let mut aur_packages = Vec::new();
        let mut apt_packages = Vec::new();
        let mut dnf_packages = Vec::new();
        let mut flatpak_packages = Vec::new();

        for package in &self.packages {
            match package.source.as_str() {
                "pacman" => pacman_packages.push(package),
                "aur" => aur_packages.push(package),
                "apt" => apt_packages.push(package),
                "dnf" => dnf_packages.push(package),
                "flatpak" => flatpak_packages.push(package),
                _ => {}
            }
        }

        PackageStats {
            total: pacman_packages.len()
                + aur_packages.len()
                + apt_packages.len()
                + dnf_packages.len()
                + flatpak_packages.len(),
            pacman: pacman_packages.len(),
            aur: aur_packages.len(),
            apt: apt_packages.len(),
            dnf: dnf_packages.len(),
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
    pub dnf: usize,
    pub flatpak: usize,
    pub outdated: usize,
}
