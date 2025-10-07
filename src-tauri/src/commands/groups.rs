use crate::models::PackageInfo;
use std::process::Command;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PackageGroup {
    pub name: String,
    pub packages: Vec<String>,
    pub installed_count: usize,
    pub total_count: usize,
}

/// List all available package groups
#[tauri::command]
pub async fn list_groups() -> Result<Vec<PackageGroup>, String> {
    // Get list of group names
    let output = Command::new("/usr/bin/pacman")
        .args(&["-Sg"])
        .output()
        .map_err(|e| format!("Failed to execute pacman -Sg: {}", e))?;

    if !output.status.success() {
        return Err(format!("pacman -Sg failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Extract unique group names (pacman -Sg without args returns only group names)
    let mut group_names: Vec<String> = stdout
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    
    group_names.sort();
    group_names.dedup();
    
    if group_names.is_empty() {
        return Err("No package groups found on your system".to_string());
    }
    
    // For each group, get its packages (this is slower but correct)
    let mut groups_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    
    for group_name in &group_names {
        let group_output = Command::new("/usr/bin/pacman")
            .args(&["-Sg", group_name])
            .output()
            .map_err(|e| format!("Failed to get packages for group {}: {}", group_name, e))?;
        
        let group_stdout = String::from_utf8_lossy(&group_output.stdout);
        let packages: Vec<String> = group_stdout
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(parts[1].to_string())
                } else {
                    None
                }
            })
            .collect();
        
        if !packages.is_empty() {
            groups_map.insert(group_name.clone(), packages);
        }
    }

    // Get installed packages
    let installed_output = Command::new("/usr/bin/pacman")
        .args(&["-Q"])
        .output()
        .map_err(|e| format!("Failed to get installed packages: {}", e))?;

    let installed_stdout = String::from_utf8_lossy(&installed_output.stdout);
    let installed: std::collections::HashSet<String> = installed_stdout
        .lines()
        .filter_map(|line| line.split_whitespace().next().map(|s| s.to_string()))
        .collect();

    // Build result
    let mut groups: Vec<PackageGroup> = groups_map
        .into_iter()
        .map(|(name, packages)| {
            let installed_count = packages.iter().filter(|p| installed.contains(*p)).count();
            let total_count = packages.len();
            PackageGroup {
                name,
                packages,
                installed_count,
                total_count,
            }
        })
        .collect();

    groups.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(groups)
}

/// Get packages in a specific group
#[tauri::command]
pub async fn get_group_packages(group: String) -> Result<Vec<PackageInfo>, String> {
    let output = Command::new("/usr/bin/pacman")
        .args(&["-Sg", &group])
        .output()
        .map_err(|e| format!("Failed to get group packages: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let package_names: Vec<String> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                Some(parts[1].to_string())
            } else {
                None
            }
        })
        .collect();

    // Get installed status
    let installed_output = Command::new("/usr/bin/pacman")
        .args(&["-Q"])
        .output()
        .map_err(|e| format!("Failed to get installed packages: {}", e))?;

    let installed_stdout = String::from_utf8_lossy(&installed_output.stdout);
    let installed: std::collections::HashSet<String> = installed_stdout
        .lines()
        .filter_map(|line| line.split_whitespace().next().map(|s| s.to_string()))
        .collect();

    // Get package info
    let mut packages = Vec::new();
    for pkg_name in package_names {
        let is_installed = installed.contains(&pkg_name);
        packages.push(PackageInfo {
            name: pkg_name.clone(),
            version: String::new(),
            repo: group.clone(),
            description: format!("Member of {} group", group),
            installed: is_installed,
        });
    }

    Ok(packages)
}

/// Install a package group
#[tauri::command]
pub async fn install_group(window: tauri::Window, group: String) -> Result<crate::models::CommandResult, String> {
    crate::pacman::operations::install_package_async(window, group).await
}
