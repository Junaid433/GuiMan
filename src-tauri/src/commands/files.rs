use std::process::Command;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FileInfo {
    pub path: String,
    pub package: String,
    pub is_directory: bool,
}

/// List all files owned by a package
#[tauri::command]
pub async fn list_package_files(package: String) -> Result<Vec<FileInfo>, String> {
    let output = Command::new("/usr/bin/pacman")
        .args(["-Ql", &package])
        .output()
        .map_err(|e| format!("Failed to list package files: {}", e))?;

    if !output.status.success() {
        return Err(format!("Package '{}' not found", package));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let files: Vec<FileInfo> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let path = parts[1].trim().to_string();
                let is_directory = path.ends_with('/');
                Some(FileInfo {
                    path,
                    package: package.clone(),
                    is_directory,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(files)
}

/// Find which package owns a file
#[tauri::command]
pub async fn find_file_owner(file_path: String) -> Result<String, String> {
    let output = Command::new("/usr/bin/pacman")
        .args(["-Qo", &file_path])
        .output()
        .map_err(|e| format!("Failed to find file owner: {}", e))?;

    if !output.status.success() {
        return Err(format!("File '{}' is not owned by any package", file_path));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Output format: "/path/to/file is owned by package-name 1.2.3"
    if let Some(owned_by) = stdout.split(" is owned by ").nth(1) {
        if let Some(package) = owned_by.split_whitespace().next() {
            return Ok(package.to_string());
        }
    }

    Err("Could not parse package owner".to_string())
}

/// Search for files matching a pattern across all packages
#[tauri::command]
pub async fn search_files(pattern: String) -> Result<Vec<FileInfo>, String> {
    let output = Command::new("/usr/bin/pacman")
        .args(["-Fl"])
        .output()
        .map_err(|e| format!("Failed to search files: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let pattern_lower = pattern.to_lowercase();

    let files: Vec<FileInfo> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let package = parts[0].to_string();
                let path = parts[1].to_string();

                if path.to_lowercase().contains(&pattern_lower) {
                    let is_directory = path.ends_with('/');
                    return Some(FileInfo {
                        path,
                        package,
                        is_directory,
                    });
                }
            }
            None
        })
        .take(100) // Limit results
        .collect();

    Ok(files)
}

/// Get backup files for a package
#[tauri::command]
pub async fn list_package_backups(package: String) -> Result<Vec<String>, String> {
    let output = Command::new("/usr/bin/pacman")
        .args(["-Qii", &package])
        .output()
        .map_err(|e| format!("Failed to get package backups: {}", e))?;

    if !output.status.success() {
        return Err(format!("Package '{}' not found", package));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut in_backup_section = false;
    let mut backups = Vec::new();

    for line in stdout.lines() {
        if line.starts_with("Backup Files") {
            in_backup_section = true;
            continue;
        }

        if in_backup_section {
            if line.starts_with("       ") || line.starts_with("\t") {
                let backup_file = line.trim().to_string();
                if !backup_file.is_empty() {
                    backups.push(backup_file);
                }
            } else if !line.trim().is_empty() {
                break; // End of backup section
            }
        }
    }

    Ok(backups)
}
