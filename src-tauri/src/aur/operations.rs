use crate::models::PackageInfo;
use crate::utils::is_command_available;
use std::process::Command;

/// Get package information from AUR
pub fn get_aur_package_info(package: &str, helper: &str) -> Result<String, String> {
    let helper_cmd = match helper {
        "yay" => "yay",
        "paru" => "paru",
        _ => "yay",
    };

    if !is_command_available(helper_cmd) {
        return Err(format!("{} is not installed", helper_cmd));
    }

    let output = Command::new(helper_cmd)
        .args(&["-Si", package])
        .output()
        .map_err(|e| format!("Failed to get AUR package info: {}", e))?;

    if !output.status.success() || output.stdout.is_empty() {
        return Err(format!("Package {} not found in AUR", package));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// List AUR packages
pub fn list_aur_packages(helper: &str) -> Result<Vec<PackageInfo>, String> {
    let helper_cmd = match helper {
        "yay" => "yay",
        "paru" => "paru",
        _ => return Err("Invalid AUR helper".to_string()),
    };

    if !is_command_available(helper_cmd) {
        return Err(format!("{} is not installed. Please install it first.", helper_cmd));
    }

    let output = Command::new(helper_cmd)
        .args(&["-Sl", "aur"])
        .output()
        .map_err(|e| format!("Failed to list AUR packages: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut packages = Vec::new();

    for line in stdout.lines().take(50) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let name = parts[1];
            let version = parts[2];
            let installed = line.contains("[installed]");

            packages.push(PackageInfo {
                name: name.to_string(),
                version: version.to_string(),
                repo: "aur".to_string(),
                description: String::new(),
                installed,
            });
        }
    }

    Ok(packages)
}

