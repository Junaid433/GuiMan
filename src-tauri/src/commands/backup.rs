use serde::{Deserialize, Serialize};
use std::process::Command;
use std::fs;
use std::path::Path;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageBackup {
    pub name: String,
    pub timestamp: String,
    pub description: String,
    pub explicit_packages: Vec<String>,
    pub aur_packages: Vec<String>,
    pub total_packages: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageHook {
    pub name: String,
    pub path: String,
    pub description: String,
    pub triggers: Vec<String>,
    pub enabled: bool,
}

/// Create a backup of currently installed packages
#[tauri::command]
pub async fn create_package_backup(name: String, description: String) -> Result<String, String> {
    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let backup_name = if name.is_empty() {
        format!("backup_{}", timestamp)
    } else {
        name
    };
    
    // Get explicit packages (user-installed)
    let explicit_output = Command::new("/usr/bin/pacman")
        .args(&["-Qe"])
        .output()
        .map_err(|e| format!("Failed to get explicit packages: {}", e))?;
    
    if !explicit_output.status.success() {
        return Err("Failed to query explicit packages".to_string());
    }
    
    let explicit_packages: Vec<String> = String::from_utf8_lossy(&explicit_output.stdout)
        .lines()
        .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    // Get AUR packages
    let mut aur_packages = Vec::new();
    for package in &explicit_packages {
        let output = Command::new("/usr/bin/pacman")
            .args(&["-Qi", package])
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.starts_with("Repository") {
                        let repo = line.split(':').nth(1).unwrap_or("").trim();
                        if repo == "local" {
                            // Check if it's an AUR package by trying yay/paru
                            if is_aur_package(package).await {
                                aur_packages.push(package.clone());
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    
    let backup = PackageBackup {
        name: backup_name.clone(),
        timestamp: timestamp.clone(),
        description,
        explicit_packages: explicit_packages.clone(),
        aur_packages,
        total_packages: explicit_packages.len(),
    };
    
    // Save backup to file
    let backup_dir = get_backup_directory()?;
    fs::create_dir_all(&backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;
    
    let backup_file = format!("{}/{}.json", backup_dir, backup_name);
    let backup_json = serde_json::to_string_pretty(&backup)
        .map_err(|e| format!("Failed to serialize backup: {}", e))?;
    
    fs::write(&backup_file, backup_json)
        .map_err(|e| format!("Failed to write backup file: {}", e))?;
    
    // Also create traditional pacman list files
    let pacman_list = format!("{}/{}.txt", backup_dir, backup_name);
    let package_list = explicit_packages.join("\n");
    fs::write(&pacman_list, package_list)
        .map_err(|e| format!("Failed to write package list: {}", e))?;
    
    Ok(format!("Backup '{}' created successfully with {} packages", backup_name, explicit_packages.len()))
}

/// List all available backups
#[tauri::command]
pub async fn list_backups() -> Result<Vec<PackageBackup>, String> {
    let backup_dir = get_backup_directory()?;
    
    if !Path::new(&backup_dir).exists() {
        return Ok(Vec::new());
    }
    
    let mut backups = Vec::new();
    let entries = fs::read_dir(&backup_dir)
        .map_err(|e| format!("Failed to read backup directory: {}", e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(backup) = serde_json::from_str::<PackageBackup>(&content) {
                    backups.push(backup);
                }
            }
        }
    }
    
    // Sort by timestamp (newest first)
    backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    Ok(backups)
}

/// Restore packages from a backup
#[tauri::command]
pub async fn restore_package_backup(backup_name: String, install_missing: bool, remove_extra: bool) -> Result<String, String> {
    let backup_dir = get_backup_directory()?;
    let backup_file = format!("{}/{}.json", backup_dir, backup_name);
    
    if !Path::new(&backup_file).exists() {
        return Err(format!("Backup '{}' not found", backup_name));
    }
    
    let backup_content = fs::read_to_string(&backup_file)
        .map_err(|e| format!("Failed to read backup file: {}", e))?;
    
    let backup: PackageBackup = serde_json::from_str(&backup_content)
        .map_err(|e| format!("Failed to parse backup file: {}", e))?;
    
    let mut results = Vec::new();
    
    if install_missing {
        // Get currently installed explicit packages
        let current_output = Command::new("/usr/bin/pacman")
            .args(&["-Qe"])
            .output()
            .map_err(|e| format!("Failed to get current packages: {}", e))?;
        
        let current_packages: Vec<String> = String::from_utf8_lossy(&current_output.stdout)
            .lines()
            .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        // Find missing packages
        let missing_official: Vec<String> = backup.explicit_packages
            .iter()
            .filter(|pkg| !backup.aur_packages.contains(pkg) && !current_packages.contains(pkg))
            .cloned()
            .collect();
        
        let missing_aur: Vec<String> = backup.aur_packages
            .iter()
            .filter(|pkg| !current_packages.contains(pkg))
            .cloned()
            .collect();
        
        // Install missing official packages
        if !missing_official.is_empty() {
            let install_cmd = format!("pkexec pacman -S --needed --noconfirm {}", missing_official.join(" "));
            let output = Command::new("/bin/sh")
                .args(&["-c", &install_cmd])
                .output()
                .map_err(|e| format!("Failed to install official packages: {}", e))?;
            
            if output.status.success() {
                results.push(format!("Installed {} official packages", missing_official.len()));
            } else {
                results.push(format!("Failed to install some official packages: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }
        
        // Install missing AUR packages
        if !missing_aur.is_empty() {
            let aur_helper = get_aur_helper().unwrap_or("yay".to_string());
            let install_cmd = format!("{} -S --needed --noconfirm {}", aur_helper, missing_aur.join(" "));
            let output = Command::new("/bin/sh")
                .args(&["-c", &install_cmd])
                .output()
                .map_err(|e| format!("Failed to install AUR packages: {}", e))?;
            
            if output.status.success() {
                results.push(format!("Installed {} AUR packages", missing_aur.len()));
            } else {
                results.push(format!("Failed to install some AUR packages: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }
        
        if missing_official.is_empty() && missing_aur.is_empty() {
            results.push("All packages from backup are already installed".to_string());
        }
    }
    
    if remove_extra {
        // This is more complex and potentially dangerous, so we'll just provide a warning
        results.push("Note: Automatic removal of extra packages is not implemented for safety. Please review manually.".to_string());
    }
    
    Ok(results.join("\n"))
}

/// Delete a backup
#[tauri::command]
pub async fn delete_package_backup(backup_name: String) -> Result<String, String> {
    let backup_dir = get_backup_directory()?;
    let backup_file = format!("{}/{}.json", backup_dir, backup_name);
    let list_file = format!("{}/{}.txt", backup_dir, backup_name);
    
    // Remove JSON backup file
    if Path::new(&backup_file).exists() {
        fs::remove_file(&backup_file)
            .map_err(|e| format!("Failed to delete backup file: {}", e))?;
    }
    
    // Remove text list file
    if Path::new(&list_file).exists() {
        fs::remove_file(&list_file)
            .map_err(|e| format!("Failed to delete list file: {}", e))?;
    }
    
    Ok(format!("Backup '{}' deleted successfully", backup_name))
}

/// List pacman hooks
#[tauri::command]
pub async fn list_pacman_hooks() -> Result<Vec<PackageHook>, String> {
    let hook_dirs = [
        "/usr/share/libalpm/hooks",
        "/etc/pacman.d/hooks",
    ];
    
    let mut hooks = Vec::new();
    
    for hook_dir in &hook_dirs {
        if let Ok(entries) = fs::read_dir(hook_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("hook") {
                        if let Ok(content) = fs::read_to_string(&path) {
                            let hook = parse_hook_file(&path.to_string_lossy(), &content);
                            hooks.push(hook);
                        }
                    }
                }
            }
        }
    }
    
    hooks.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(hooks)
}

/// Export package list in various formats
#[tauri::command]
pub async fn export_packages(format: String) -> Result<String, String> {
    let output = Command::new("/usr/bin/pacman")
        .args(&["-Qe"])
        .output()
        .map_err(|e| format!("Failed to get package list: {}", e))?;
    
    if !output.status.success() {
        return Err("Failed to query packages".to_string());
    }
    
    let packages: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    match format.as_str() {
        "txt" => Ok(packages.join("\n")),
        "json" => {
            let json = serde_json::to_string_pretty(&packages)
                .map_err(|e| format!("Failed to serialize to JSON: {}", e))?;
            Ok(json)
        },
        "csv" => Ok(packages.join(",")),
        "pacman" => {
            // Format for pacman -S command
            Ok(format!("pacman -S {}", packages.join(" ")))
        },
        _ => Err(format!("Unsupported format: {}", format))
    }
}

// Helper functions

fn get_backup_directory() -> Result<String, String> {
    let home = std::env::var("HOME")
        .map_err(|_| "Failed to get HOME directory".to_string())?;
    Ok(format!("{}/.config/guiman/backups", home))
}

async fn is_aur_package(package: &str) -> bool {
    let aur_helpers = ["yay", "paru"];
    for helper in &aur_helpers {
        let output = Command::new(&format!("/usr/bin/{}", helper))
            .args(&["-Si", package])
            .output();
        
        if let Ok(output) = output {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("Repository") && stdout.contains("aur") {
                    return true;
                }
            }
        }
    }
    false
}

fn get_aur_helper() -> Option<String> {
    let helpers = ["yay", "paru"];
    for helper in &helpers {
        if Command::new("/usr/bin/which").arg(helper).output().is_ok() {
            return Some(helper.to_string());
        }
    }
    None
}

fn parse_hook_file(path: &str, content: &str) -> PackageHook {
    let name = Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown")
        .to_string();
    
    let mut description = String::new();
    let mut triggers = Vec::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("Description") {
            description = line.split('=').nth(1).unwrap_or("").trim().to_string();
        } else if line.starts_with("Target") {
            let target = line.split('=').nth(1).unwrap_or("").trim().to_string();
            if !target.is_empty() {
                triggers.push(target);
            }
        }
    }
    
    if description.is_empty() {
        description = "No description available".to_string();
    }
    
    PackageHook {
        name,
        path: path.to_string(),
        description,
        triggers,
        enabled: true, // Assume enabled if file exists
    }
}
