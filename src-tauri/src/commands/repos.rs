use std::process::Command;
use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub enabled: bool,
    pub package_count: usize,
    pub servers: Vec<String>,
}

/// List all repositories
#[tauri::command]
pub async fn list_repositories() -> Result<Vec<Repository>, String> {
    // Get all repos from pacman -Sl
    let output = Command::new("pacman")
        .args(&["-Sl"])
        .output()
        .map_err(|e| format!("Failed to list repositories: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut repo_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for line in stdout.lines() {
        if let Some(repo_name) = line.split_whitespace().next() {
            *repo_counts.entry(repo_name.to_string()).or_insert(0) += 1;
        }
    }

    // Read pacman.conf to get enabled repos and servers
    let conf_content = fs::read_to_string("/etc/pacman.conf")
        .unwrap_or_else(|_| String::new());

    let mut repositories = Vec::new();
    let mut current_repo: Option<String> = None;
    let mut servers = Vec::new();

    for line in conf_content.lines() {
        let trimmed = line.trim();
        
        // Check for repo section [reponame]
        if trimmed.starts_with('[') && trimmed.ends_with(']') && trimmed != "[options]" {
            // Save previous repo
            if let Some(repo_name) = current_repo.take() {
                let package_count = repo_counts.get(&repo_name).copied().unwrap_or(0);
                repositories.push(Repository {
                    name: repo_name,
                    enabled: true,
                    package_count,
                    servers: servers.clone(),
                });
                servers.clear();
            }
            
            // Start new repo
            let repo_name = trimmed[1..trimmed.len()-1].to_string();
            current_repo = Some(repo_name);
        } else if trimmed.starts_with("Server") && current_repo.is_some() {
            if let Some(server) = trimmed.split('=').nth(1) {
                servers.push(server.trim().to_string());
            }
        }
    }

    // Save last repo
    if let Some(repo_name) = current_repo {
        let package_count = repo_counts.get(&repo_name).copied().unwrap_or(0);
        repositories.push(Repository {
            name: repo_name,
            enabled: true,
            package_count,
            servers: servers.clone(),
        });
    }

    Ok(repositories)
}

/// Get packages from a specific repository
#[tauri::command]
pub async fn get_repo_packages(repo: String) -> Result<Vec<crate::models::PackageInfo>, String> {
    let output = Command::new("pacman")
        .args(&["-Sl", &repo])
        .output()
        .map_err(|e| format!("Failed to get repository packages: {}", e))?;

    if !output.status.success() {
        return Err(format!("Repository '{}' not found", repo));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Get installed packages
    let installed_output = Command::new("pacman")
        .args(&["-Q"])
        .output()
        .map_err(|e| format!("Failed to get installed packages: {}", e))?;

    let installed_stdout = String::from_utf8_lossy(&installed_output.stdout);
    let installed: std::collections::HashSet<String> = installed_stdout
        .lines()
        .filter_map(|line| line.split_whitespace().next().map(|s| s.to_string()))
        .collect();

    let packages: Vec<crate::models::PackageInfo> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let repo_name = parts[0];
                let package_name = parts[1];
                let version = parts[2];
                let is_installed = installed.contains(package_name);
                
                Some(crate::models::PackageInfo {
                    name: package_name.to_string(),
                    version: version.to_string(),
                    repo: repo_name.to_string(),
                    description: String::new(),
                    installed: is_installed,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(packages)
}

/// Sync package databases
#[tauri::command]
pub async fn sync_databases() -> Result<String, String> {
    let output = Command::new("pkexec")
        .args(&["pacman", "-Sy"])
        .output()
        .map_err(|e| format!("Failed to sync databases: {}", e))?;

    if output.status.success() {
        Ok("Databases synced successfully".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Get mirror list with status
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MirrorInfo {
    pub url: String,
    pub enabled: bool,
    pub country: String,
}

#[tauri::command]
pub async fn get_mirrorlist_info() -> Result<Vec<MirrorInfo>, String> {
    let content = fs::read_to_string("/etc/pacman.d/mirrorlist")
        .map_err(|e| format!("Failed to read mirrorlist: {}", e))?;

    let mut mirrors = Vec::new();
    let mut current_country = String::from("Unknown");

    for line in content.lines() {
        let trimmed = line.trim();
        
        // Check for country comment
        if trimmed.starts_with("##") {
            // Extract country name
            current_country = trimmed
                .trim_start_matches('#')
                .trim()
                .to_string();
        } else if trimmed.starts_with("Server") || trimmed.starts_with("#Server") {
            let enabled = !trimmed.starts_with('#');
            let url = if enabled {
                trimmed.split('=').nth(1).unwrap_or("").trim().to_string()
            } else {
                trimmed
                    .trim_start_matches('#')
                    .split('=')
                    .nth(1)
                    .unwrap_or("")
                    .trim()
                    .to_string()
            };
            
            if !url.is_empty() {
                mirrors.push(MirrorInfo {
                    url,
                    enabled,
                    country: current_country.clone(),
                });
            }
        }
    }

    Ok(mirrors)
}

/// Update mirrorlist (requires root)
#[tauri::command]
pub async fn update_mirrorlist(mirrors: Vec<MirrorInfo>) -> Result<String, String> {
    let mut content = String::new();
    let mut current_country = String::new();

    for mirror in mirrors {
        // Add country header if it changed
        if mirror.country != current_country {
            content.push_str(&format!("\n## {}\n", mirror.country));
            current_country = mirror.country.clone();
        }

        // Add server line (commented if disabled)
        if mirror.enabled {
            content.push_str(&format!("Server = {}\n", mirror.url));
        } else {
            content.push_str(&format!("#Server = {}\n", mirror.url));
        }
    }

    // Write to temp file first
    let temp_path = "/tmp/mirrorlist.new";
    fs::write(temp_path, &content)
        .map_err(|e| format!("Failed to write temp mirrorlist: {}", e))?;

    // Use pkexec to copy to system location
    let output = Command::new("pkexec")
        .args(&["cp", temp_path, "/etc/pacman.d/mirrorlist"])
        .output()
        .map_err(|e| format!("Failed to update mirrorlist: {}", e))?;

    if output.status.success() {
        Ok("Mirrorlist updated successfully".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Rank mirrors by speed (using reflector if available)
#[tauri::command]
pub async fn rank_mirrors(country: Option<String>, count: Option<usize>) -> Result<String, String> {
    // Check if reflector is installed
    let check = Command::new("which")
        .arg("reflector")
        .output()
        .map_err(|e| format!("Failed to check for reflector: {}", e))?;

    if !check.status.success() {
        return Err("Reflector is not installed. Install it with: sudo pacman -S reflector".to_string());
    }

    let mut args = vec![
        "reflector",
        "--save",
        "/etc/pacman.d/mirrorlist",
        "--protocol",
        "https",
        "--sort",
        "rate",
    ];

    let count_str;
    if let Some(n) = count {
        args.push("--latest");
        count_str = n.to_string();
        args.push(&count_str);
    }

    let country_str;
    if let Some(c) = country {
        args.push("--country");
        country_str = c;
        args.push(&country_str);
    }

    let output = Command::new("pkexec")
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to rank mirrors: {}", e))?;

    if output.status.success() {
        Ok("Mirrors ranked successfully!".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}


