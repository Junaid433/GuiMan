use crate::models::PackageInfo;
use std::process::Command;
use std::collections::HashMap;

/// List all installed packages
pub fn list_installed_packages() -> Result<Vec<PackageInfo>, String> {
    let output = Command::new("pacman")
        .args(&["-Q"])
        .output()
        .map_err(|e| format!("Failed to execute pacman: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut packages = Vec::new();
    
    // Build repository map
    let repo_map = build_repository_map()?;
    
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let pkg_name = parts[0].to_string();
            let repo = repo_map.get(&pkg_name).cloned().unwrap_or_else(|| "local".to_string());
            
            packages.push(PackageInfo {
                name: pkg_name,
                version: parts[1].to_string(),
                repo,
                description: String::new(),
                installed: true,
            });
        }
    }
    
    Ok(packages)
}

/// Build a map of package names to their repositories
fn build_repository_map() -> Result<HashMap<String, String>, String> {
    let mut repo_map = HashMap::new();
    
    let sync_output = Command::new("pacman")
        .args(&["-Sl"])
        .output()
        .ok();
    
    if let Some(sync) = sync_output {
        let sync_str = String::from_utf8_lossy(&sync.stdout);
        for sync_line in sync_str.lines() {
            let parts: Vec<&str> = sync_line.split_whitespace().collect();
            if parts.len() >= 2 {
                repo_map.insert(parts[1].to_string(), parts[0].to_string());
            }
        }
    }
    
    Ok(repo_map)
}

/// Check for package updates
pub fn check_for_updates() -> Result<Vec<PackageInfo>, String> {
    let output = Command::new("pacman")
        .args(&["-Qu"])
        .output()
        .map_err(|e| format!("Failed to check updates: {}", e))?;

    if !output.status.success() && output.stdout.is_empty() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut packages = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            let pkg_name = parts[0].to_string();
            let current_version = parts[1].to_string();
            let new_version = parts[3].to_string();
            
            let repo = get_package_repository(&pkg_name).unwrap_or_else(|| "unknown".to_string());
            
            packages.push(PackageInfo {
                name: pkg_name,
                version: new_version.clone(),
                repo,
                description: format!("{} → {}", current_version, new_version),
                installed: true,
            });
        }
    }

    Ok(packages)
}

/// Get the repository of a package
fn get_package_repository(package: &str) -> Option<String> {
    let output = Command::new("pacman")
        .args(&["-Si", package])
        .output()
        .ok()?;
    
    let info_str = String::from_utf8_lossy(&output.stdout);
    for line in info_str.lines() {
        if line.starts_with("Repository") {
            return Some(
                line.split(':')
                    .nth(1)?
                    .trim()
                    .to_string()
            );
        }
    }
    None
}

/// List orphaned packages
pub fn list_orphan_packages() -> Result<Vec<String>, String> {
    let output = Command::new("pacman")
        .args(&["-Qdtq"])
        .output()
        .map_err(|e| format!("Failed to execute pacman: {}", e))?;

    if !output.status.success() && output.stdout.is_empty() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout
        .lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect())
}

/// Get package history from pacman log
pub fn get_package_history() -> Result<Vec<PackageInfo>, String> {
    let output = Command::new("tail")
        .args(&["-n", "500", "/var/log/pacman.log"])
        .output()
        .map_err(|e| format!("Failed to read pacman log: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut history = Vec::new();

    for line in stdout.lines() {
        if line.contains("installed") || line.contains("removed") || line.contains("upgraded") {
            if let Some(parsed) = parse_log_entry(line) {
                history.push(parsed);
            }
        }
    }

    history.reverse();
    history.truncate(100);
    Ok(history)
}

/// Parse a single pacman log entry
fn parse_log_entry(line: &str) -> Option<PackageInfo> {
    let parts: Vec<&str> = line.split(']').collect();
    if parts.len() < 2 {
        return None;
    }

    let timestamp = parts[0].trim_start_matches('[').trim();
    let content = parts[1].trim();

    if content.contains("installed") {
        let pkg_parts: Vec<&str> = content.split_whitespace().collect();
        if pkg_parts.len() >= 3 {
            return Some(PackageInfo {
                name: pkg_parts[2].to_string(),
                version: if pkg_parts.len() > 3 {
                    pkg_parts[3].trim_matches('(').trim_matches(')').to_string()
                } else {
                    String::new()
                },
                repo: "history".to_string(),
                description: format!("Installed at {}", timestamp),
                installed: false,
            });
        }
    } else if content.contains("removed") {
        let pkg_parts: Vec<&str> = content.split_whitespace().collect();
        if pkg_parts.len() >= 3 {
            return Some(PackageInfo {
                name: pkg_parts[2].to_string(),
                version: if pkg_parts.len() > 3 {
                    pkg_parts[3].trim_matches('(').trim_matches(')').to_string()
                } else {
                    String::new()
                },
                repo: "history".to_string(),
                description: format!("Removed at {}", timestamp),
                installed: false,
            });
        }
    } else if content.contains("upgraded") {
        let pkg_parts: Vec<&str> = content.split_whitespace().collect();
        if pkg_parts.len() >= 4 {
            return Some(PackageInfo {
                name: pkg_parts[2].to_string(),
                version: pkg_parts[4].trim_matches('(').trim_matches(')').to_string(),
                repo: "history".to_string(),
                description: format!("Upgraded at {} from {}", timestamp, pkg_parts[3]),
                installed: false,
            });
        }
    }

    None
}

/// Get detailed package information
pub fn get_package_info(package: &str, repo: &str, is_installed: bool) -> Result<String, String> {
    let output = if is_installed {
        Command::new("pacman")
            .args(&["-Qi", package])
            .output()
            .map_err(|e| format!("Failed to get package info: {}", e))?
    } else if repo == "aur" {
        // This will be handled by AUR module
        return Err("Use AUR module for AUR packages".to_string());
    } else {
        Command::new("pacman")
            .args(&["-Si", package])
            .output()
            .map_err(|e| format!("Failed to get package info: {}", e))?
    };

    if !output.status.success() || output.stdout.is_empty() {
        return Err(format!("No info found for package: {}", package));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Export list of explicitly installed packages
pub fn export_package_list() -> Result<Vec<String>, String> {
    let output = Command::new("pacman")
        .args(&["-Qqe"])
        .output()
        .map_err(|e| format!("Failed to get package list: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

/// Get cache size
pub fn get_cache_size() -> Result<String, String> {
    let output = Command::new("du")
        .args(&["-sh", "/var/cache/pacman/pkg"])
        .output()
        .map_err(|e| format!("Failed to get cache size: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout
        .split_whitespace()
        .next()
        .unwrap_or("Unknown")
        .to_string())
}

