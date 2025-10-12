use crate::models::{PackageInfo, CommandResult};
use crate::utils::is_command_available;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tauri::Window;
use tokio;
use serde_json;

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

    let output = Command::new(format!("/usr/bin/{}", helper_cmd))
        .args(["-Si", package])
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

    let output = Command::new(format!("/usr/bin/{}", helper_cmd))
        .args(["-Sl", "aur"])
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

/// Vote for an AUR package
pub fn vote_aur_package(package: &str, helper: &str) -> Result<String, String> {
    let helper_cmd = match helper {
        "yay" => "yay",
        "paru" => "paru",
        _ => return Err("Invalid AUR helper".to_string()),
    };

    if !is_command_available(helper_cmd) {
        return Err(format!("{} is not installed", helper_cmd));
    }

    // Check if package exists in AUR first
    let output = Command::new(format!("/usr/bin/{}", helper_cmd))
        .args(["-Si", package])
        .output()
        .map_err(|e| format!("Failed to check package: {}", e))?;

    if !output.status.success() {
        return Err(format!("Package {} not found in AUR", package));
    }

    // Since voting requires AUR account and web interface, provide helpful message
    let aur_url = format!("https://aur.archlinux.org/packages/{}", package);
    Err(format!("To vote for this package:\n1. Visit: {}\n2. Log in with your AUR account\n3. Click the 'Vote' button\n\nNote: You need an AUR account to vote.", aur_url))
}

/// Flag AUR package as out-of-date
pub fn flag_aur_package(package: &str, helper: &str, comment: Option<String>) -> Result<String, String> {
    let helper_cmd = match helper {
        "yay" => "yay",
        "paru" => "paru", 
        _ => return Err("Invalid AUR helper".to_string()),
    };

    if !is_command_available(helper_cmd) {
        return Err(format!("{} is not installed", helper_cmd));
    }

    // Check if package exists in AUR first
    let output = Command::new(format!("/usr/bin/{}", helper_cmd))
        .args(["-Si", package])
        .output()
        .map_err(|e| format!("Failed to check package: {}", e))?;

    if !output.status.success() {
        return Err(format!("Package {} not found in AUR", package));
    }

    // Flagging requires web interface
    let aur_url = format!("https://aur.archlinux.org/packages/{}", package);
    let message = if let Some(comment) = comment {
        format!("To flag this package as out-of-date:\n1. Visit: {}\n2. Log in with your AUR account\n3. Click 'Flag Package Out-of-Date'\n4. Add your comment: {}\n\nNote: Only flag if the package is actually outdated.", aur_url, comment)
    } else {
        format!("To flag this package as out-of-date:\n1. Visit: {}\n2. Log in with your AUR account\n3. Click 'Flag Package Out-of-Date'\n\nNote: Only flag if the package is actually outdated.", aur_url)
    };
    
    Err(message)
}

/// Adopt an AUR package (for maintainers)
pub fn adopt_aur_package(package: &str, helper: &str) -> Result<String, String> {
    let helper_cmd = match helper {
        "yay" => "yay",
        "paru" => "paru",
        _ => return Err("Invalid AUR helper".to_string()),
    };

    if !is_command_available(helper_cmd) {
        return Err(format!("{} is not installed", helper_cmd));
    }

    // Check if package exists in AUR first
    let output = Command::new(format!("/usr/bin/{}", helper_cmd))
        .args(["-Si", package])
        .output()
        .map_err(|e| format!("Failed to check package: {}", e))?;

    if !output.status.success() {
        return Err(format!("Package {} not found in AUR", package));
    }

    // Check if package is orphaned by looking for "Maintainer: None"
    let stdout = String::from_utf8_lossy(&output.stdout);
    let is_orphaned = stdout.contains("Maintainer") && stdout.contains("None");
    
    let aur_url = format!("https://aur.archlinux.org/packages/{}", package);
    
    if is_orphaned {
        Err(format!("To adopt this orphaned package:\n1. Visit: {}\n2. Log in with your AUR account\n3. Click 'Adopt Package'\n\nNote: You will become the maintainer and be responsible for updates.", aur_url))
    } else {
        Err(format!("This package already has a maintainer.\nVisit: {} to see package details.\n\nNote: Only orphaned packages can be adopted.", aur_url))
    }
}

/// Get AUR package build options and dependencies
pub fn get_aur_build_info(package: &str, helper: &str) -> Result<String, String> {
    let helper_cmd = match helper {
        "yay" => "yay",
        "paru" => "paru",
        _ => return Err("Invalid AUR helper".to_string()),
    };

    if !is_command_available(helper_cmd) {
        return Err(format!("{} is not installed", helper_cmd));
    }

    // Get detailed package information including build dependencies
    let output = Command::new(format!("/usr/bin/{}", helper_cmd))
        .args(["-Si", package])
        .output()
        .map_err(|e| format!("Failed to get build info: {}", e))?;

    if !output.status.success() {
        return Err(format!("Failed to get build info for {}", package));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Install AUR package with custom build options
pub fn install_aur_with_options(package: &str, helper: &str, options: Vec<String>) -> Result<String, String> {
    let helper_cmd = match helper {
        "yay" => "yay",
        "paru" => "paru",
        _ => return Err("Invalid AUR helper".to_string()),
    };

    if !is_command_available(helper_cmd) {
        return Err(format!("{} is not installed", helper_cmd));
    }

    let mut args = vec!["-S", "--needed", "--noconfirm", package];
    for option in &options {
        args.push(option);
    }

    let output = Command::new("/usr/bin/pkexec")
        .arg(format!("/usr/bin/{}", helper_cmd))
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to install with options: {}", e))?;

    if output.status.success() {
        Ok(format!("Successfully installed {} with custom options", package))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Install an AUR package asynchronously with real-time output
pub async fn install_aur_package_async(window: Window, package: String) -> Result<CommandResult, String> {
    let pkg_clone = package.clone();
    let helper = "yay".to_string();

    tokio::spawn(async move {
        let helper_cmd = match helper.as_str() {
            "yay" => "yay",
            "paru" => "paru",
            _ => {
                let _ = window.emit(
                    "install-complete",
                    serde_json::json!({
                        "success": false,
                        "message": "✗ Invalid AUR helper specified"
                    })
                );
                return;
            }
        };

        if !is_command_available(helper_cmd) {
            let _ = window.emit(
                "install-complete",
                serde_json::json!({
                    "success": false,
                    "message": format!("✗ {} is not installed", helper_cmd)
                })
            );
            return;
        }

        let child = Command::new("/usr/bin/pkexec")
            .args([format!("/usr/bin/{}", helper_cmd), "-S".to_string(), "--needed".to_string(), "--noconfirm".to_string(), pkg_clone.clone()])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match child {
            Ok(mut child_process) => {
                // Handle stdout
                if let Some(stdout) = child_process.stdout.take() {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines().map_while(Result::ok) {
                        let _ = window.emit("install-output", line);
                    }
                }

                // Handle stderr
                if let Some(stderr) = child_process.stderr.take() {
                    let reader = BufReader::new(stderr);
                    for line in reader.lines().map_while(Result::ok) {
                        let _ = window.emit("install-output", format!("ERROR: {}", line));
                    }
                }

                let result = match child_process.wait() {
                    Ok(result) => result,
                    Err(e) => {
                        let _ = window.emit("install-complete", serde_json::json!({
                            "success": false,
                            "message": format!("Failed to wait for AUR install process: {}", e)
                        }));
                        return;
                    }
                };

                let success = result.success();
                let message = if success {
                    format!("✓ Installation of {} completed successfully!", pkg_clone)
                } else {
                    format!("✗ Installation of {} failed!", pkg_clone)
                };

                let _ = window.emit(
                    "install-complete",
                    serde_json::json!({
                        "success": success,
                        "message": message
                    })
                );
            }
            Err(_) => {
                let _ = window.emit(
                    "install-complete",
                    serde_json::json!({
                        "success": false,
                        "message": format!("✗ Failed to start AUR installation for {}", pkg_clone)
                    })
                );
            }
        }
    });

    Ok(CommandResult::success(format!("AUR installation of {} started", package)))
}

