use rand::Rng;

pub fn generate_funny_comment(package_count: usize) -> String {
    let comments = vec![
        format!("{} packages installed. System is happy.", package_count),
        format!("{} packages installed. Don't forget Ctrl+Z!", package_count),
        format!("{} packages. The system is getting fat.", package_count),
        "You installed Vim again... what now?".to_string(),
        "All your packages are belong to us.".to_string(),
        "Package addiction is real.".to_string(),
    ];
    
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..comments.len());
    comments[index].clone()
}

pub fn format_package_name(name: &str) -> String {
    // Clean up package names for display
    name.split_whitespace().next().unwrap_or(name).to_string()
}

pub fn format_version(version: &str) -> String {
    if version.is_empty() {
        "unknown".to_string()
    } else {
        version.to_string()
    }
}
