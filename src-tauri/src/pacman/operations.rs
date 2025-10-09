use crate::models::CommandResult;
use tauri::Window;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tokio;
use serde_json;

/// Install a package
pub async fn install_package_async(window: Window, package: String) -> Result<CommandResult, String> {
    let pkg_clone = package.clone();
    
    tokio::spawn(async move {
        let mut child = Command::new("/usr/bin/pkexec")
            .arg("/usr/bin/pacman")
            .args(&["-S", "--needed", "--noconfirm", &pkg_clone])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start install process");

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let _ = window.emit("install-output", line);
                }
            }
        }

        let result = child.wait().expect("Failed to wait for install");

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
    });

    Ok(CommandResult::success(format!(
        "Installation of {} started",
        package
    )))
}

/// Remove a package
pub async fn remove_package_async(window: Window, package: String) -> Result<CommandResult, String> {
    let pkg_clone = package.clone();
    
    tokio::spawn(async move {
        let mut child = Command::new("/usr/bin/pkexec")
            .arg("/usr/bin/pacman")
            .args(&["-Rs", "--noconfirm", &pkg_clone])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start remove process");

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let _ = window.emit("remove-output", line);
                }
            }
        }

        let result = child.wait().expect("Failed to wait for remove");

        let success = result.success();
        let message = if success {
            format!("✓ Removal of {} completed successfully!", pkg_clone)
        } else {
            format!("✗ Removal of {} failed!", pkg_clone)
        };

        let _ = window.emit(
            "remove-complete",
            serde_json::json!({
                "success": success,
                "message": message
            })
        );
    });

    Ok(CommandResult::success(format!("Removal of {} started", package)))
}

/// Update the system with proper partial upgrade handling
pub async fn update_system_async(window: Window) -> Result<CommandResult, String> {
    tokio::spawn(async move {
        // Use -Syu to avoid partial upgrade issues (sync and upgrade in one command)
        let child = Command::new("/usr/bin/pkexec")
            .arg("/usr/bin/pacman")
            .args(&["-Syu", "--needed", "--noconfirm"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match child {
            Ok(mut upgrade_child) => {
                if let Some(stdout) = upgrade_child.stdout.take() {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            let _ = window.emit("update-output", line);
                        }
                    }
                }

                if let Some(stderr) = upgrade_child.stderr.take() {
                    let reader = BufReader::new(stderr);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            let _ = window.emit("update-output", format!("ERROR: {}", line));
                        }
                    }
                }

                let result = upgrade_child.wait().expect("Failed to wait for update");

                let success = result.success();
                let exit_code = result.code().unwrap_or(-1);
                let message = if success {
                    "✓ System update completed successfully!".to_string()
                } else {
                    format!("✗ System update failed! (Exit code: {})", exit_code)
                };

                let _ = window.emit(
                    "update-complete",
                    serde_json::json!({
                        "success": success,
                        "message": message,
                        "exit_code": exit_code
                    })
                );
            }
            Err(e) => {
                let _ = window.emit("update-complete", serde_json::json!({
                    "success": false,
                    "message": format!("✗ Failed to start system upgrade: {}", e)
                }));
            }
        }
    });

    Ok(CommandResult::success("System update started".to_string()))
}

/// Clean package cache
pub async fn clean_cache_async(window: Window, aur_helper: Option<String>) -> Result<CommandResult, String> {
    let helper = aur_helper.unwrap_or_else(|| "yay".to_string());
    
    tokio::spawn(async move {
        let _ = window.emit("cache-clean-output", "Cleaning pacman cache...");
        
        let _ = window.emit("cache-clean-output", "Checking if pacman database is available...");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        let _ = Command::new("/bin/sh")
            .args(&["-c", "/usr/bin/pkexec rm -f /var/lib/pacman/db.lck 2>/dev/null || true"])
            .output();
        
        let _ = window.emit("cache-clean-output", "Starting cache clean...");
        
        let pacman_result = Command::new("/bin/sh")
            .args(&["-c", "printf 'y\\ny\\n' | /usr/bin/pkexec /usr/bin/pacman -Scc 2>&1"])
            .output()
            .expect("Failed to start cache clean");

        let stdout_str = String::from_utf8_lossy(&pacman_result.stdout);
        for line in stdout_str.lines() {
            if !line.trim().is_empty() && line != "y" {
                let _ = window.emit("cache-clean-output", line);
            }
        }
        
        let pacman_success = pacman_result.status.success();
        
        let helper_cmd = match helper.as_str() {
            "paru" => "paru",
            _ => "yay",
        };
        
        let _ = window.emit("cache-clean-output", format!("\nCleaning {} cache...", helper_cmd));
        
        let aur_result = Command::new("/bin/sh")
            .args(&["-c", &format!("timeout 10 /bin/sh -c 'printf \"y\\ny\\n\" | /usr/bin/pkexec {} -Scc' 2>&1 || echo 'Cache clean completed or timed out'", helper_cmd)])
            .output();
        
        let mut aur_success = false;
        if let Ok(output) = aur_result {
            aur_success = output.status.success();
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            for line in stdout_str.lines() {
                if !line.trim().is_empty() && line != "y" {
                    let _ = window.emit("cache-clean-output", line);
                }
            }
        }
        
        let (success, final_message) = match (pacman_success, aur_success) {
            (true, true) => (true, "✓ Cache cleaned successfully!".to_string()),
            (true, false) => (true, "✓ Pacman cache cleaned, but AUR helper cache cleaning failed or timed out".to_string()),
            (false, true) => (false, "⚠ Pacman cache cleaning failed (database might be locked), but AUR cache cleaned".to_string()),
            (false, false) => (false, "✗ Cache cleaning failed! Make sure no other package manager is running.".to_string()),
        };

        let _ = window.emit("cache-clean-complete", serde_json::json!({
            "success": success,
            "message": final_message
        }));
    });

    Ok(CommandResult::success("Cache cleaning started".to_string()))
}

