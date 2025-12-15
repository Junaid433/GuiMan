use crate::models::{CommandResult, PackageInfo};
use crate::{aur, pacman, utils};
use serde::{Deserialize, Serialize};
use serde_json;
use std::process::Command;
use tauri::Window;

#[derive(Serialize, Deserialize)]
pub struct PackageCounts {
    pub total: u32,
    pub aur: u32,
}

#[derive(Serialize, Deserialize)]
pub struct EnhancedPackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub licenses: Vec<String>,
    pub groups: Vec<String>,
    pub provides: Vec<String>,
    pub depends_on: Vec<String>,
    pub optional_deps: Vec<String>,
    pub required_by: Vec<String>,
    pub optional_for: Vec<String>,
    pub conflicts_with: Vec<String>,
    pub replaces: Vec<String>,
    pub installed_size: String,
    pub download_size: String,
    pub packager: String,
    pub build_date: String,
    pub install_date: String,
    pub install_reason: String,
    pub install_script: bool,
    pub validated_by: String,
}

#[tauri::command]
pub async fn get_enhanced_package_info(pkg: String) -> Result<EnhancedPackageInfo, String> {
    // Try installed package first
    let output = Command::new("/usr/bin/pacman")
        .args(["-Qi", &pkg])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        // Try repo package
        let repo_output = Command::new("/usr/bin/pacman")
            .args(["-Si", &pkg])
            .output()
            .map_err(|e| e.to_string())?;
        
        if repo_output.status.success() {
            String::from_utf8_lossy(&repo_output.stdout).to_string()
        } else {
            return Err(format!("Package {} not found", pkg));
        }
    };

    // Parse the output
    let mut info = EnhancedPackageInfo {
        name: pkg.clone(),
        version: String::new(),
        description: String::new(),
        url: String::new(),
        licenses: Vec::new(),
        groups: Vec::new(),
        provides: Vec::new(),
        depends_on: Vec::new(),
        optional_deps: Vec::new(),
        required_by: Vec::new(),
        optional_for: Vec::new(),
        conflicts_with: Vec::new(),
        replaces: Vec::new(),
        installed_size: String::new(),
        download_size: String::new(),
        packager: String::new(),
        build_date: String::new(),
        install_date: String::new(),
        install_reason: String::new(),
        install_script: false,
        validated_by: String::new(),
    };

    for line in stdout.lines() {
        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim();
            let value = line[pos + 1..].trim();

            match key {
                "Name" => info.name = value.to_string(),
                "Version" => info.version = value.to_string(),
                "Description" => info.description = value.to_string(),
                "URL" => info.url = value.to_string(),
                "Licenses" => info.licenses = parse_list(value),
                "Groups" => info.groups = parse_list(value),
                "Provides" => info.provides = parse_list(value),
                "Depends On" => info.depends_on = parse_list(value),
                "Optional Deps" => info.optional_deps = parse_list(value),
                "Required By" => info.required_by = parse_list(value),
                "Optional For" => info.optional_for = parse_list(value),
                "Conflicts With" => info.conflicts_with = parse_list(value),
                "Replaces" => info.replaces = parse_list(value),
                "Installed Size" => info.installed_size = value.to_string(),
                "Download Size" => info.download_size = value.to_string(),
                "Packager" => info.packager = value.to_string(),
                "Build Date" => info.build_date = value.to_string(),
                "Install Date" => info.install_date = value.to_string(),
                "Install Reason" => info.install_reason = value.to_string(),
                "Install Script" => info.install_script = value == "Yes",
                "Validated By" => info.validated_by = value.to_string(),
                _ => {}
            }
        }
    }

    Ok(info)
}

fn parse_list(value: &str) -> Vec<String> {
    if value == "None" || value.is_empty() {
        return Vec::new();
    }
    value.split_whitespace().map(|s| s.to_string()).collect()
}

#[tauri::command]
pub async fn get_package_counts() -> Result<PackageCounts, String> {
    // Get total package count efficiently using pacman -Q | wc -l
    let total_output = Command::new("sh")
        .args(["-c", "pacman -Qq | wc -l"])
        .output()
        .map_err(|e| e.to_string())?;

    let total = String::from_utf8_lossy(&total_output.stdout)
        .trim()
        .parse::<u32>()
        .unwrap_or(0);

    // Get AUR package count (foreign packages not in official repos)
    let aur_output = Command::new("sh")
        .args(["-c", "pacman -Qmq | wc -l"])
        .output()
        .map_err(|e| e.to_string())?;

    let aur = String::from_utf8_lossy(&aur_output.stdout)
        .trim()
        .parse::<u32>()
        .unwrap_or(0);

    Ok(PackageCounts { total, aur })
}

#[tauri::command]
pub async fn list_installed() -> Result<Vec<PackageInfo>, String> {
    pacman::list_installed_packages()
}

#[tauri::command]
pub async fn check_updates() -> Result<Vec<PackageInfo>, String> {
    pacman::check_for_updates()
}

#[tauri::command]
pub async fn list_orphans() -> Result<Vec<String>, String> {
    pacman::list_orphan_packages()
}

#[tauri::command]
pub async fn get_package_history() -> Result<Vec<PackageInfo>, String> {
    pacman::get_package_history()
}

#[tauri::command]
pub async fn get_package_info(
    pkg: String,
    repo: Option<String>,
    is_installed: Option<bool>,
) -> Result<serde_json::Value, String> {
    let repo = repo.unwrap_or_else(|| "unknown".to_string());
    let is_installed = is_installed.unwrap_or(false);

    let stdout = if repo == "aur" && !is_installed {
        aur::get_aur_package_info(&pkg, "yay")?
    } else {
        pacman::get_package_info(&pkg, &repo, is_installed)?
    };

    let info = utils::parse_package_info(&stdout);
    Ok(serde_json::Value::Object(info))
}

#[tauri::command]
pub async fn list_aur_packages(helper: String) -> Result<Vec<PackageInfo>, String> {
    aur::list_aur_packages(&helper)
}

#[tauri::command]
pub async fn install_package(window: Window, pkg: String) -> Result<CommandResult, String> {
    // Check if package exists in official repositories
    let is_official = Command::new("/usr/bin/pacman")
        .args(["-Si", &pkg])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if is_official {
        // Use pacman for official packages
        pacman::install_package_async(window, pkg).await
    } else {
        // Use AUR helper for AUR packages (async with real-time output)
        aur::install_aur_package_async(window, pkg).await
    }
}

#[tauri::command]
pub async fn remove_package(window: Window, pkg: String) -> Result<CommandResult, String> {
    // Check if package exists in official repositories
    let is_official = Command::new("/usr/bin/pacman")
        .args(["-Si", &pkg])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);

    if is_official {
        // Use pacman for official packages
        pacman::remove_package_async(window, pkg).await
    } else {
        // Use AUR helper for AUR packages - we need to create this function
        aur::remove_aur_package_async(window, pkg).await
    }
}

// AUR Advanced Features

#[tauri::command]
pub async fn vote_aur_package(package: String, helper: String) -> Result<String, String> {
    aur::vote_aur_package(&package, &helper)
}

#[tauri::command]
pub async fn flag_aur_package(
    package: String,
    helper: String,
    comment: Option<String>,
) -> Result<String, String> {
    aur::flag_aur_package(&package, &helper, comment)
}

#[tauri::command]
pub async fn adopt_aur_package(package: String, helper: String) -> Result<String, String> {
    aur::adopt_aur_package(&package, &helper)
}

#[tauri::command]
pub async fn get_aur_build_info(package: String, helper: String) -> Result<String, String> {
    aur::get_aur_build_info(&package, &helper)
}

#[tauri::command]
pub async fn install_aur_with_options(
    package: String,
    helper: String,
    options: Vec<String>,
) -> Result<String, String> {
    aur::install_aur_with_options(&package, &helper, options)
}
