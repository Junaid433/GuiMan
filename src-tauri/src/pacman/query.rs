use crate::models::PackageInfo;
use std::process::Command;
use std::collections::HashMap;

/// List all installed packages
pub fn list_installed_packages() -> Result<Vec<PackageInfo>, String> {
    let output = Command::new("/usr/bin/pacman")
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
    
    let sync_output = Command::new("/usr/bin/pacman")
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
    let output = Command::new("/usr/bin/pacman")
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
    let output = Command::new("/usr/bin/pacman")
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
    let output = Command::new("/usr/bin/pacman")
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
    let output = Command::new("/usr/bin/tail")
        .args(&["-n", "500", "/var/log/pacman.log"])
        .output()
        .map_err(|e| format!("Failed to read pacman log: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut history = Vec::new();

    for line in stdout.lines() {
        if line.contains("[ALPM]") && (line.contains("installed") || line.contains("removed") || line.contains("upgraded")) {
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
    // Format: [2025-10-02T14:00:05+0600] [ALPM] installed lsof (4.99.5-2)
    let parts: Vec<&str> = line.split("] [ALPM] ").collect();
    if parts.len() != 2 {
        return None;
    }

    let timestamp = parts[0].trim_start_matches('[').trim();
    let content = parts[1].trim();

    if content.starts_with("installed ") {
        // Format: "installed lsof (4.99.5-2)"
        let rest = content.strip_prefix("installed ").unwrap_or("");
        if let Some(open_paren) = rest.find(" (") {
            let name = rest[..open_paren].to_string();
            let version_part = &rest[open_paren + 2..];
            let version = version_part.trim_end_matches(')').to_string();
            
            return Some(PackageInfo {
                name,
                version,
                repo: "log".to_string(),
                description: format!("📦 Installed on {}", format_timestamp(timestamp)),
                installed: false,
            });
        }
    } else if content.starts_with("removed ") {
        // Format: "removed package-name (version)"
        let rest = content.strip_prefix("removed ").unwrap_or("");
        if let Some(open_paren) = rest.find(" (") {
            let name = rest[..open_paren].to_string();
            let version_part = &rest[open_paren + 2..];
            let version = version_part.trim_end_matches(')').to_string();
            
            return Some(PackageInfo {
                name,
                version,
                repo: "log".to_string(),
                description: format!("🗑️ Removed on {}", format_timestamp(timestamp)),
                installed: false,
            });
        }
    } else if content.starts_with("upgraded ") {
        // Format: "upgraded package-name (old-version -> new-version)"
        let rest = content.strip_prefix("upgraded ").unwrap_or("");
        if let Some(open_paren) = rest.find(" (") {
            let name = rest[..open_paren].to_string();
            let version_part = &rest[open_paren + 2..];
            let version_info = version_part.trim_end_matches(')');
            
            // Extract new version (after ->)
            let version = if let Some(arrow_pos) = version_info.find(" -> ") {
                version_info[arrow_pos + 4..].to_string()
            } else {
                version_info.to_string()
            };
            
            return Some(PackageInfo {
                name,
                version,
                repo: "log".to_string(),
                description: format!("⬆️ Upgraded on {}", format_timestamp(timestamp)),
                installed: false,
            });
        }
    }

    None
}

/// Format timestamp for display
fn format_timestamp(timestamp: &str) -> String {
    // Convert ISO timestamp to readable format
    timestamp.split('T').next().unwrap_or(timestamp).to_string()
}

/// Get detailed package information
pub fn get_package_info(package: &str, repo: &str, is_installed: bool) -> Result<String, String> {
    let output = if is_installed {
        Command::new("/usr/bin/pacman")
            .args(&["-Qi", package])
            .output()
            .map_err(|e| format!("Failed to get package info: {}", e))?
    } else if repo == "aur" {
        // This will be handled by AUR module
        return Err("Use AUR module for AUR packages".to_string());
    } else {
        Command::new("/usr/bin/pacman")
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
    let output = Command::new("/usr/bin/pacman")
        .args(&["-Qqe"])
        .output()
        .map_err(|e| format!("Failed to get package list: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

/// Get cache size
pub fn get_cache_size() -> Result<String, String> {
    let output = Command::new("/usr/bin/du")
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

