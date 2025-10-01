#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tauri::Window;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PackageInfo {
    name: String,
    version: String,
    repo: String,
    description: String,
    installed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommandResult {
    success: bool,
    message: String,
    data: Option<serde_json::Value>,
}

#[tauri::command]
async fn search_package(query: String) -> Result<Vec<PackageInfo>, String> {
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    let output = Command::new("pacman")
        .args(&["-Ss", &query])
        .output()
        .map_err(|e| format!("Failed to execute pacman: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut packages = Vec::new();
    let lines: Vec<&str> = stdout.lines().collect();
    
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.contains('/') {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name_parts: Vec<&str> = parts[0].split('/').collect();
                let repo = name_parts.get(0).unwrap_or(&"unknown").to_string();
                let name = name_parts.get(1).unwrap_or(&parts[0]).to_string();
                let version = parts[1].to_string();
                
                let description = if i + 1 < lines.len() {
                    lines[i + 1].trim().to_string()
                } else {
                    String::new()
                };

                let installed = line.contains("[installed]");
                
                packages.push(PackageInfo {
                    name,
                    version,
                    repo,
                    description,
                    installed,
                });
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    Ok(packages)
}

#[tauri::command]
async fn list_installed() -> Result<Vec<PackageInfo>, String> {
    let output = Command::new("pacman")
        .args(&["-Q"])
        .output()
        .map_err(|e| format!("Failed to execute pacman: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut packages = Vec::new();
    
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let output_info = Command::new("pacman")
                .args(&["-Qi", parts[0]])
                .output()
                .ok();

            let repo = if let Some(info) = output_info {
                let info_str = String::from_utf8_lossy(&info.stdout);
                info_str.lines()
                    .find(|l| l.starts_with("Repository"))
                    .and_then(|l| l.split(':').nth(1))
                    .map(|r| r.trim().to_string())
                    .unwrap_or_else(|| "local".to_string())
            } else {
                "local".to_string()
            };

            packages.push(PackageInfo {
                name: parts[0].to_string(),
                version: parts[1].to_string(),
                repo,
                description: String::new(),
                installed: true,
            });
        }
    }

    Ok(packages)
}

#[tauri::command]
async fn install_package(window: Window, pkg: String) -> Result<CommandResult, String> {
    let pkg_clone = pkg.clone();
    tokio::spawn(async move {
        let mut child = Command::new("pkexec")
            .args(&["pacman", "-S", "--noconfirm", &pkg_clone])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start install process");

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let _ = window.emit("install-log", line);
                }
            }
        }

        let status = child.wait().expect("Failed to wait for child");
        let _ = window.emit("install-complete", status.success());
    });

    Ok(CommandResult {
        success: true,
        message: format!("Installation of {} started", pkg),
        data: None,
    })
}

#[tauri::command]
async fn remove_package(window: Window, pkg: String) -> Result<CommandResult, String> {
    let pkg_clone = pkg.clone();
    tokio::spawn(async move {
        let mut child = Command::new("pkexec")
            .args(&["pacman", "-R", "--noconfirm", &pkg_clone])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start remove process");

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let _ = window.emit("remove-log", line);
                }
            }
        }

        let status = child.wait().expect("Failed to wait for child");
        let _ = window.emit("remove-complete", status.success());
    });

    Ok(CommandResult {
        success: true,
        message: format!("Removal of {} started", pkg),
        data: None,
    })
}

#[tauri::command]
async fn update_system(window: Window) -> Result<CommandResult, String> {
    tokio::spawn(async move {
        let mut child = Command::new("pkexec")
            .args(&["pacman", "-Syu", "--noconfirm"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start update process");

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let _ = window.emit("update-log", line);
                }
            }
        }

        let status = child.wait().expect("Failed to wait for child");
        let _ = window.emit("update-complete", status.success());
    });

    Ok(CommandResult {
        success: true,
        message: "System update started".to_string(),
        data: None,
    })
}

#[tauri::command]
async fn list_orphans() -> Result<Vec<String>, String> {
    let output = Command::new("pacman")
        .args(&["-Qdtq"])
        .output()
        .map_err(|e| format!("Failed to execute pacman: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

#[tauri::command]
async fn get_package_history() -> Result<Vec<String>, String> {
    let output = Command::new("tail")
        .args(&["-n", "100", "/var/log/pacman.log"])
        .output()
        .map_err(|e| format!("Failed to read pacman log: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

#[tauri::command]
async fn check_updates() -> Result<Vec<PackageInfo>, String> {
    let output = Command::new("checkupdates")
        .output()
        .map_err(|e| format!("Failed to check updates: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut packages = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            packages.push(PackageInfo {
                name: parts[0].to_string(),
                version: parts[3].to_string(),
                repo: "updates".to_string(),
                description: format!("{} -> {}", parts[1], parts[3]),
                installed: true,
            });
        }
    }

    Ok(packages)
}

#[tauri::command]
async fn clean_cache(window: Window) -> Result<CommandResult, String> {
    tokio::spawn(async move {
        let mut child = Command::new("pkexec")
            .args(&["pacman", "-Sc", "--noconfirm"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start cache clean process");

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let _ = window.emit("cache-clean-log", line);
                }
            }
        }

        let status = child.wait().expect("Failed to wait for child");
        let _ = window.emit("cache-clean-complete", status.success());
    });

    Ok(CommandResult {
        success: true,
        message: "Cache cleaning started".to_string(),
        data: None,
    })
}

#[tauri::command]
async fn get_package_info(pkg: String) -> Result<serde_json::Value, String> {
    let output = Command::new("pacman")
        .args(&["-Qi", &pkg])
        .output()
        .map_err(|e| format!("Failed to get package info: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    let mut info = serde_json::Map::new();
    
    for line in stdout.lines() {
        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim().to_lowercase().replace(' ', "_");
            let value = line[pos + 1..].trim().to_string();
            info.insert(key, serde_json::Value::String(value));
        }
    }

    Ok(serde_json::Value::Object(info))
}

#[tauri::command]
async fn export_package_list() -> Result<Vec<String>, String> {
    let output = Command::new("pacman")
        .args(&["-Qqe"])
        .output()
        .map_err(|e| format!("Failed to get package list: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

#[tauri::command]
async fn get_cache_size() -> Result<String, String> {
    let output = Command::new("du")
        .args(&["-sh", "/var/cache/pacman/pkg"])
        .output()
        .map_err(|e| format!("Failed to get cache size: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let size = stdout.split_whitespace().next().unwrap_or("0").to_string();
    Ok(size)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            search_package,
            install_package,
            remove_package,
            update_system,
            list_installed,
            list_orphans,
            get_package_history,
            check_updates,
            clean_cache,
            get_package_info,
            export_package_list,
            get_cache_size
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
