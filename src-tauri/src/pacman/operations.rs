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
        let mut child = match Command::new("/usr/bin/pkexec")
            .arg("/usr/bin/pacman")
            .args(["-S", "--needed", "--noconfirm", &pkg_clone])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
            Ok(child) => child,
            Err(e) => {
                let _ = window.emit("install-complete", serde_json::json!({
                    "success": false,
                    "message": format!("Failed to start install process: {}", e)
                }));
                return;
            }
        };

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                let _ = window.emit("install-log", line);
            }
        }

        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
                let _ = window.emit("install-log", format!("ERROR: {}", line));
            }
        }

        let result = match child.wait() {
            Ok(result) => result,
            Err(e) => {
                let _ = window.emit("install-complete", serde_json::json!({
                    "success": false,
                    "message": format!("Failed to wait for install process: {}", e)
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
        let mut child = match Command::new("/usr/bin/pkexec")
            .arg("/usr/bin/pacman")
            .args(["-Rs", "--noconfirm", &pkg_clone])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn() {
            Ok(child) => child,
            Err(e) => {
                let _ = window.emit("remove-complete", serde_json::json!({
                    "success": false,
                    "message": format!("Failed to start remove process: {}", e)
                }));
                return;
            }
        };

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                let _ = window.emit("remove-log", line);
            }
        }

        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
                let _ = window.emit("remove-log", format!("ERROR: {}", line));
            }
        }

        let result = match child.wait() {
            Ok(result) => result,
            Err(e) => {
                let _ = window.emit("remove-complete", serde_json::json!({
                    "success": false,
                    "message": format!("Failed to wait for remove process: {}", e)
                }));
                return;
            }
        };

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
            .args(["-Syu", "--needed", "--noconfirm"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        match child {
            Ok(mut upgrade_child) => {
                // Take the handles first
                let stdout = upgrade_child.stdout.take();
                let stderr = upgrade_child.stderr.take();

                // Stream stdout and stderr concurrently
                let window_clone1 = window.clone();
                let window_clone2 = window.clone();
                let stdout_handle = tokio::spawn(async move {
                    if let Some(stdout) = stdout {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines().map_while(Result::ok) {
                            let _ = window_clone1.emit("update-log", line);
                        }
                    }
                });

                let stderr_handle = tokio::spawn(async move {
                    if let Some(stderr) = stderr {
                        let reader = BufReader::new(stderr);
                        for line in reader.lines().map_while(Result::ok) {
                            let _ = window_clone2.emit("update-log", format!("ERROR: {}", line));
                        }
                    }
                });

                // Wait for both streaming tasks to complete
                let _ = tokio::join!(stdout_handle, stderr_handle);

                let result = match upgrade_child.wait() {
                    Ok(result) => result,
                    Err(e) => {
                        let _ = window.emit("update-complete", serde_json::json!({
                            "success": false,
                            "message": format!("Failed to wait for update process: {}", e)
                        }));
                        return;
                    }
                };

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
        // Remove database lock if it exists
        let _ = Command::new("/bin/sh")
            .args(["-c", "/usr/bin/pkexec rm -f /var/lib/pacman/db.lck 2>/dev/null || true"])
            .output();

        // Clean pacman cache with real-time output
        let pacman_child = Command::new("/usr/bin/pkexec")
            .args(["/usr/bin/pacman", "-Scc", "--noconfirm"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut pacman_success = false;
        if let Ok(mut child) = pacman_child {
            // Stream stdout
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines().map_while(Result::ok) {
                    let _ = window.emit("cache-clean-output", line);
                }
            }

            // Stream stderr
            if let Some(stderr) = child.stderr.take() {
                let reader = BufReader::new(stderr);
                for line in reader.lines().map_while(Result::ok) {
                    let _ = window.emit("cache-clean-output", format!("ERROR: {}", line));
                }
            }

            if let Ok(result) = child.wait() {
                pacman_success = result.success();
            }
        }

        let helper_cmd = match helper.as_str() {
            "paru" => "paru",
            _ => "yay",
        };

        // Clean AUR helper cache with real-time output
        let aur_child = Command::new(format!("/usr/bin/{}", helper_cmd))
            .args(["-Scc", "--noconfirm"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut aur_success = false;
        if let Ok(mut child) = aur_child {
            // Stream stdout
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines().map_while(Result::ok) {
                    let _ = window.emit("cache-clean-output", line);
                }
            }

            // Stream stderr
            if let Some(stderr) = child.stderr.take() {
                let reader = BufReader::new(stderr);
                for line in reader.lines().map_while(Result::ok) {
                    let _ = window.emit("cache-clean-output", format!("ERROR: {}", line));
                }
            }

            if let Ok(result) = child.wait() {
                aur_success = result.success();
            }
        }

        let (success, final_message) = match (pacman_success, aur_success) {
            (true, true) => (true, "✓ Cache cleaned successfully!".to_string()),
            (true, false) => (true, "✓ Pacman cache cleaned, but AUR helper cache cleaning failed".to_string()),
            (false, true) => (false, "⚠ Pacman cache cleaning failed, but AUR cache cleaned".to_string()),
            (false, false) => (false, "✗ Cache cleaning failed!".to_string()),
        };

        let _ = window.emit("cache-clean-complete", serde_json::json!({
            "success": success,
            "message": final_message
        }));
    });

    Ok(CommandResult::success("Cache cleaning started".to_string()))
}

