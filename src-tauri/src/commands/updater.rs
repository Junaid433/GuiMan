use serde::{Deserialize, Serialize};
use std::process::Command;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub changelog: String,
    pub download_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProgress {
    pub stage: String,
    pub progress: u32,
    pub message: String,
    pub completed: bool,
    pub error: Option<String>,
}

/// Check for application updates
#[tauri::command]
pub async fn check_app_updates() -> Result<UpdateInfo, String> {
    let current_version = env!("CARGO_PKG_VERSION").to_string();
    
    // DEMO MODE: Simulate an available update
    let demo_mode = std::env::var("GUIMAN_DEMO_UPDATE").is_ok();
    
    if demo_mode {
        // Create a demo update
        return Ok(UpdateInfo {
            current_version: current_version.clone(),
            latest_version: "1.1.0".to_string(),
            update_available: true,
            changelog: "🚀 Demo Update v1.1.0\n\n✨ New Features:\n• Automatic update system\n• Enhanced dependency graphs\n• Improved AUR integration\n• Better performance\n\n🐛 Bug Fixes:\n• Fixed history parsing\n• Improved error handling\n• UI/UX improvements\n\n🔧 Technical:\n• Updated dependencies\n• Code optimizations\n• Security enhancements".to_string(),
            download_url: "https://github.com/Junaid433/guiman/releases/download/v1.1.0/guiman-demo.AppImage".to_string(),
        });
    }
    
    // Real GitHub API check
    let github_api_url = "https://api.github.com/repos/Junaid433/guiman/releases/latest";
    
    let client = reqwest::Client::new();
    let response = client
        .get(github_api_url)
        .header("User-Agent", "GuiMan-Updater")
        .send()
        .await
        .map_err(|e| format!("Failed to check for updates: {}", e))?;
    
    if !response.status().is_success() {
        return Err("Failed to fetch release information".to_string());
    }
    
    let release_data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse release data: {}", e))?;
    
    let latest_version = release_data["tag_name"]
        .as_str()
        .unwrap_or(&current_version)
        .trim_start_matches('v')
        .to_string();
    
    let changelog = release_data["body"]
        .as_str()
        .unwrap_or("No changelog available")
        .to_string();
    
    let download_url = release_data["assets"]
        .as_array()
        .and_then(|assets| assets.first())
        .and_then(|asset| asset["browser_download_url"].as_str())
        .unwrap_or("")
        .to_string();
    
    let update_available = version_compare(&latest_version, &current_version);
    
    Ok(UpdateInfo {
        current_version,
        latest_version,
        update_available,
        changelog,
        download_url,
    })
}

/// Download and install update automatically
#[tauri::command]
pub async fn install_app_update(window: tauri::Window) -> Result<String, String> {
    // Emit progress updates to frontend
    let emit_progress = |stage: &str, progress: u32, message: &str| {
        let _ = window.emit("update-progress", UpdateProgress {
            stage: stage.to_string(),
            progress,
            message: message.to_string(),
            completed: false,
            error: None,
        });
    };
    
    emit_progress("checking", 10, "Checking for updates...");
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    let update_info = check_app_updates().await?;
    
    if !update_info.update_available {
        return Ok("No updates available".to_string());
    }
    
    // DEMO MODE: Simulate update process
    let demo_mode = std::env::var("GUIMAN_DEMO_UPDATE").is_ok();
    
    if demo_mode {
        emit_progress("downloading", 20, "Connecting to GitHub...");
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
        
        emit_progress("downloading", 35, "Downloading GuiMan v1.1.0...");
        tokio::time::sleep(tokio::time::Duration::from_millis(1200)).await;
        
        emit_progress("downloading", 55, "Download complete (25.4 MB)");
        tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;
        
        emit_progress("installing", 70, "Verifying update integrity...");
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
        
        emit_progress("installing", 85, "Creating backup of current version...");
        tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;
        
        emit_progress("installing", 95, "Installing new version...");
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        
        emit_progress("completed", 100, "🎉 Demo update completed successfully!");
        
        let _ = window.emit("update-progress", UpdateProgress {
            stage: "completed".to_string(),
            progress: 100,
            message: "Demo update installed! This was just a simulation - no actual files were changed.".to_string(),
            completed: true,
            error: None,
        });
        
        return Ok("Demo update completed successfully! (No actual files were modified)".to_string());
    }
    
    // Real update process
    emit_progress("downloading", 30, "Downloading update...");
    
    // Create temp directory for download
    let temp_dir = std::env::temp_dir().join("guiman-update");
    fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    
    // Download the update
    let client = reqwest::Client::new();
    let response = client
        .get(&update_info.download_url)
        .send()
        .await
        .map_err(|e| format!("Failed to download update: {}", e))?;
    
    if !response.status().is_success() {
        return Err("Failed to download update file".to_string());
    }
    
    emit_progress("downloading", 60, "Saving update file...");
    
    let update_file = temp_dir.join("guiman-update.AppImage");
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read update data: {}", e))?;
    
    fs::write(&update_file, bytes)
        .map_err(|e| format!("Failed to save update file: {}", e))?;
    
    emit_progress("installing", 80, "Installing update...");
    
    // Make the file executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&update_file)
            .map_err(|e| format!("Failed to get file permissions: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&update_file, perms)
            .map_err(|e| format!("Failed to set file permissions: {}", e))?;
    }
    
    // Get current executable path
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e))?;
    
    emit_progress("installing", 90, "Replacing application...");
    
    // Create backup of current version
    let backup_path = current_exe.with_extension("backup");
    if current_exe.exists() {
        fs::copy(&current_exe, &backup_path)
            .map_err(|e| format!("Failed to create backup: {}", e))?;
    }
    
    // Replace current executable with new version
    fs::copy(&update_file, &current_exe)
        .map_err(|e| format!("Failed to replace executable: {}", e))?;
    
    // Clean up temp files
    let _ = fs::remove_file(&update_file);
    
    emit_progress("completed", 100, "Update installed successfully!");
    
    // Emit completion
    let _ = window.emit("update-progress", UpdateProgress {
        stage: "completed".to_string(),
        progress: 100,
        message: "Update completed! Please restart the application.".to_string(),
        completed: true,
        error: None,
    });
    
    Ok(format!("Successfully updated to version {}", update_info.latest_version))
}

/// Restart application after update
#[tauri::command]
pub async fn restart_application() -> Result<(), String> {
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable: {}", e))?;
    
    // Start new instance
    Command::new(&current_exe)
        .spawn()
        .map_err(|e| format!("Failed to restart application: {}", e))?;
    
    // Exit current instance
    std::process::exit(0);
}

/// Check if running from AppImage
#[tauri::command]
pub async fn is_appimage() -> Result<bool, String> {
    Ok(std::env::var("APPIMAGE").is_ok())
}

/// Get update channel (stable/beta)
#[tauri::command]
pub async fn get_update_channel() -> Result<String, String> {
    // Check if this is a beta version
    let version = env!("CARGO_PKG_VERSION");
    if version.contains("beta") || version.contains("alpha") || version.contains("rc") {
        Ok("beta".to_string())
    } else {
        Ok("stable".to_string())
    }
}

/// Enable/disable automatic updates
#[tauri::command]
pub async fn set_auto_update(enabled: bool) -> Result<(), String> {
    let config_dir = dirs::config_dir()
        .ok_or("Failed to get config directory")?
        .join("guiman");
    
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    
    let config_file = config_dir.join("updater.json");
    let config = serde_json::json!({
        "auto_update": enabled,
        "last_check": chrono::Utc::now().to_rfc3339()
    });
    
    fs::write(&config_file, serde_json::to_string_pretty(&config).unwrap())
        .map_err(|e| format!("Failed to save updater config: {}", e))?;
    
    Ok(())
}

/// Check if auto-update is enabled
#[tauri::command]
pub async fn get_auto_update_setting() -> Result<bool, String> {
    let config_dir = dirs::config_dir()
        .ok_or("Failed to get config directory")?
        .join("guiman");
    
    let config_file = config_dir.join("updater.json");
    
    if !config_file.exists() {
        return Ok(true); // Default to enabled
    }
    
    let config_content = fs::read_to_string(&config_file)
        .map_err(|e| format!("Failed to read updater config: {}", e))?;
    
    let config: serde_json::Value = serde_json::from_str(&config_content)
        .map_err(|e| format!("Failed to parse updater config: {}", e))?;
    
    Ok(config["auto_update"].as_bool().unwrap_or(true))
}

// Helper functions

fn version_compare(latest: &str, current: &str) -> bool {
    // Simple version comparison (assumes semantic versioning)
    let latest_parts: Vec<u32> = latest.split('.').filter_map(|s| s.parse().ok()).collect();
    let current_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();
    
    for i in 0..std::cmp::max(latest_parts.len(), current_parts.len()) {
        let latest_part = latest_parts.get(i).unwrap_or(&0);
        let current_part = current_parts.get(i).unwrap_or(&0);
        
        if latest_part > current_part {
            return true;
        } else if latest_part < current_part {
            return false;
        }
    }
    
    false
}
