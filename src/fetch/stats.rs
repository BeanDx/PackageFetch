use crate::fetch::PackageInfo;

pub fn count_packages_by_source(packages: &[PackageInfo]) -> PackageCounts {
    let mut counts = PackageCounts::default();
    
    for package in packages {
        match package.source.as_str() {
            "pacman" => counts.pacman += 1,
            "aur" => counts.aur += 1,
            "apt" => counts.apt += 1,
            "dnf" => counts.dnf += 1,
            "flatpak" => counts.flatpak += 1,
            _ => counts.unknown += 1,
        }
    }
    
    counts.total = counts.pacman + counts.aur + counts.apt + counts.dnf + counts.flatpak + counts.unknown;
    counts
}

#[derive(Debug, Default)]
pub struct PackageCounts {
    pub total: usize,
    pub pacman: usize,
    pub aur: usize,
    pub apt: usize,
    pub dnf: usize,
    pub flatpak: usize,
    pub unknown: usize,
}
