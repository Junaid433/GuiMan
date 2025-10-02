#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Read};
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
    
    let sync_output = Command::new("pacman")
        .args(&["-Sl"])
        .output()
        .ok();
    
    let mut repo_map = std::collections::HashMap::new();
    if let Some(sync) = sync_output {
        let sync_str = String::from_utf8_lossy(&sync.stdout);
        for sync_line in sync_str.lines() {
            let parts: Vec<&str> = sync_line.split_whitespace().collect();
            if parts.len() >= 2 {
                repo_map.insert(parts[1].to_string(), parts[0].to_string());
            }
        }
    }
    
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let pkg_name = parts[0].to_string();
            let repo = repo_map.get(&pkg_name)
                .cloned()
                .unwrap_or_else(|| "local".to_string());
            
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

        let window_clone = window.clone();
        if let Some(stdout) = child.stdout.take() {
            tokio::spawn(async move {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let _ = window_clone.emit("install-log", line);
                    }
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let window_clone = window.clone();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stderr);
                let mut buffer = Vec::new();
                let mut temp = [0u8; 1024];
                
                loop {
                    match reader.read(&mut temp) {
                        Ok(0) => break,
                        Ok(n) => {
                            for &byte in &temp[..n] {
                                if byte == b'\n' || byte == b'\r' {
                                    if !buffer.is_empty() {
                                        if let Ok(line) = String::from_utf8(buffer.clone()) {
                                            let trimmed = line.trim();
                                            if !trimmed.is_empty() {
                                                let _ = window_clone.emit("install-log", trimmed.to_string());
                                            }
                                        }
                                        buffer.clear();
                                    }
                                } else {
                                    buffer.push(byte);
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
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

        let window_clone = window.clone();
        if let Some(stdout) = child.stdout.take() {
            tokio::spawn(async move {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let _ = window_clone.emit("remove-log", line);
                    }
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let window_clone = window.clone();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stderr);
                let mut buffer = Vec::new();
                let mut temp = [0u8; 1024];
                
                loop {
                    match reader.read(&mut temp) {
                        Ok(0) => break,
                        Ok(n) => {
                            for &byte in &temp[..n] {
                                if byte == b'\n' || byte == b'\r' {
                                    if !buffer.is_empty() {
                                        if let Ok(line) = String::from_utf8(buffer.clone()) {
                                            let trimmed = line.trim();
                                            if !trimmed.is_empty() {
                                                let _ = window_clone.emit("remove-log", trimmed.to_string());
                                            }
                                        }
                                        buffer.clear();
                                    }
                                } else {
                                    buffer.push(byte);
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
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

        let window_clone = window.clone();
        if let Some(stdout) = child.stdout.take() {
            tokio::spawn(async move {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let _ = window_clone.emit("update-log", line);
                    }
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let window_clone = window.clone();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stderr);
                let mut buffer = Vec::new();
                let mut temp = [0u8; 1024];
                
                loop {
                    match reader.read(&mut temp) {
                        Ok(0) => break,
                        Ok(n) => {
                            for &byte in &temp[..n] {
                                if byte == b'\n' || byte == b'\r' {
                                    if !buffer.is_empty() {
                                        if let Ok(line) = String::from_utf8(buffer.clone()) {
                                            let trimmed = line.trim();
                                            if !trimmed.is_empty() {
                                                let _ = window_clone.emit("update-log", trimmed.to_string());
                                            }
                                        }
                                        buffer.clear();
                                    }
                                } else {
                                    buffer.push(byte);
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
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
    let output = Command::new("pacman")
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
            
            let repo_output = Command::new("pacman")
                .args(&["-Si", &pkg_name])
                .output()
                .ok();
            
            let mut repo = String::from("unknown");
            if let Some(info) = repo_output {
                let info_str = String::from_utf8_lossy(&info.stdout);
                for info_line in info_str.lines() {
                    if info_line.starts_with("Repository") {
                        repo = info_line.split(':')
                            .nth(1)
                            .unwrap_or("unknown")
                            .trim()
                            .to_string();
                        break;
                    }
                }
            }
            
            packages.push(PackageInfo {
                name: pkg_name,
                version: new_version.clone(),
                repo,
                description: format!("Installed: {} → Update: {}", current_version, new_version),
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

        let window_clone = window.clone();
        if let Some(stdout) = child.stdout.take() {
            tokio::spawn(async move {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let _ = window_clone.emit("cache-clean-log", line);
                    }
                }
            });
        }

        if let Some(stderr) = child.stderr.take() {
            let window_clone = window.clone();
            tokio::spawn(async move {
                let mut reader = BufReader::new(stderr);
                let mut buffer = Vec::new();
                let mut temp = [0u8; 1024];
                
                loop {
                    match reader.read(&mut temp) {
                        Ok(0) => break,
                        Ok(n) => {
                            for &byte in &temp[..n] {
                                if byte == b'\n' || byte == b'\r' {
                                    if !buffer.is_empty() {
                                        if let Ok(line) = String::from_utf8(buffer.clone()) {
                                            let trimmed = line.trim();
                                            if !trimmed.is_empty() {
                                                let _ = window_clone.emit("cache-clean-log", trimmed.to_string());
                                            }
                                        }
                                        buffer.clear();
                                    }
                                } else {
                                    buffer.push(byte);
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
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
    let installed_output = Command::new("pacman")
        .args(&["-Qi", &pkg])
        .output();

    let output = if let Ok(out) = installed_output {
        if out.status.success() && !out.stdout.is_empty() {
            out
        } else {
            Command::new("pacman")
                .args(&["-Si", &pkg])
                .output()
                .map_err(|e| format!("Failed to get package info: {}", e))?
        }
    } else {
        Command::new("pacman")
            .args(&["-Si", &pkg])
            .output()
            .map_err(|e| format!("Failed to get package info: {}", e))?
    };

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

#[tauri::command]
async fn install_polkit_policy() -> Result<CommandResult, String> {
    let policy_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE policyconfig PUBLIC
 "-//freedesktop//DTD PolicyKit Policy Configuration 1.0//EN"
 "http://www.freedesktop.org/standards/PolicyKit/1/policyconfig.dtd">
<policyconfig>

  <action id="com.guiman.pacman">
    <description>Run pacman package manager</description>
    <message>Authentication is required to manage packages</message>
    <icon_name>package-x-generic</icon_name>
    <defaults>
      <allow_any>auth_admin</allow_any>
      <allow_inactive>auth_admin</allow_inactive>
      <allow_active>yes</allow_active>
    </defaults>
    <annotate key="org.freedesktop.policykit.exec.path">/usr/bin/pacman</annotate>
    <annotate key="org.freedesktop.policykit.exec.allow_gui">true</annotate>
  </action>

</policyconfig>
"#;

    let temp_file = "/tmp/com.guiman.pkexec.policy";
    std::fs::write(temp_file, policy_content)
        .map_err(|e| format!("Failed to write policy file: {}", e))?;

    let output = Command::new("pkexec")
        .args(&["cp", temp_file, "/usr/share/polkit-1/actions/com.guiman.pkexec.policy"])
        .output()
        .map_err(|e| format!("Failed to install policy: {}", e))?;

    std::fs::remove_file(temp_file).ok();

    if output.status.success() {
        Ok(CommandResult {
            success: true,
            message: "Polkit policy installed successfully! Password prompts disabled.".to_string(),
            data: None,
        })
    } else {
        Err("Failed to install polkit policy. Make sure you entered the correct password.".to_string())
    }
}

#[tauri::command]
async fn check_polkit_policy() -> Result<bool, String> {
    Ok(std::path::Path::new("/usr/share/polkit-1/actions/com.guiman.pkexec.policy").exists())
}

#[tauri::command]
async fn get_popular_packages() -> Result<Vec<PackageInfo>, String> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let popular_packages = vec![
        "firefox", "chromium", "brave-bin", "git", "vim", "neovim", "code", "docker", "python",
        "rust", "nodejs", "npm", "yarn", "nginx", "apache", "redis", "postgresql", "mysql",
        "mariadb", "mongodb", "vlc", "mpv", "gimp", "inkscape", "blender", "libreoffice-fresh",
        "steam", "discord", "telegram-desktop", "spotify", "obs-studio", "kdenlive", "audacity",
        "htop", "neofetch", "btop", "tmux", "zsh", "fish", "alacritty", "kitty", "gnome-terminal",
        "konsole", "dolphin", "nautilus", "thunar", "ranger", "nnn", "mc", "wget", "curl",
        "openssh", "rsync", "rclone", "zip", "unzip", "p7zip", "tar", "gzip", "bzip2", "xz",
        "gcc", "clang", "make", "cmake", "meson", "ninja", "gdb", "valgrind", "lldb", "go",
        "kotlin", "java-runtime-common", "jdk-openjdk", "ruby", "php", "perl", "lua", "r",
        "julia", "octave", "texlive-core", "pandoc", "hugo", "jekyll", "nginx-mainline",
        "bind", "dnsmasq", "wireguard-tools", "openvpn", "nmap", "wireshark-qt", "tcpdump",
        "ettercap", "metasploit", "john", "hashcat", "aircrack-ng", "hydra", "sqlmap",
        "firefox-developer-edition", "thunderbird", "evolution", "geary", "mutt", "neomutt",
        "gnome-shell", "plasma-desktop", "xfce4", "i3-wm", "sway", "hyprland", "bspwm", "awesome",
        "polybar", "waybar", "rofi", "dmenu", "dunst", "mako", "picom", "nitrogen", "feh",
        "sddm", "lightdm", "gdm", "xorg-server", "wayland", "mesa", "vulkan-radeon", "nvidia",
        "intel-media-driver", "xf86-video-amdgpu", "cups", "sane", "hplip", "bluez", "pipewire",
        "pulseaudio", "alsa-utils", "pavucontrol", "easyeffects", "gparted", "timeshift",
        "grub", "systemd-boot", "refind", "linux", "linux-zen", "linux-lts", "linux-hardened",
        "base-devel", "multilib-devel", "networkmanager", "iwd", "dhcpcd", "netctl", "bind",
        "transmission-gtk", "qbittorrent", "deluge", "aria2", "filezilla", "remmina", "virt-manager",
        "virtualbox", "qemu", "wine", "proton", "lutris", "gamemode", "mangohud", "goverlay",
        "ansible", "terraform", "kubectl", "helm", "minikube", "docker-compose", "podman",
        "k9s", "lazydocker", "vagrant", "packer", "awscli", "gcloud-cli", "azure-cli",
        "bitwarden", "keepassxc", "pass", "gnupg", "openssl", "age", "sops", "vault"
    ];

    let mut rng = thread_rng();
    let mut shuffled = popular_packages.clone();
    shuffled.shuffle(&mut rng);
    
    let selected: Vec<&str> = shuffled.iter().take(20).copied().collect();
    
    let mut result = Vec::new();

    for pkg_name in selected {
        let output = Command::new("pacman")
            .args(&["-Ss", &format!("^{}$", pkg_name)])
            .output()
            .ok();

        if let Some(search_result) = output {
            let stdout = String::from_utf8_lossy(&search_result.stdout);
            let lines: Vec<&str> = stdout.lines().collect();
            
            if lines.len() >= 2 {
                let first_line = lines[0];
                if first_line.contains('/') {
                    let parts: Vec<&str> = first_line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let name_parts: Vec<&str> = parts[0].split('/').collect();
                        let repo = name_parts.get(0).unwrap_or(&"unknown").to_string();
                        let name = name_parts.get(1).unwrap_or(&parts[0]).to_string();
                        let version = parts[1].to_string();
                        let description = lines[1].trim().to_string();
                        let installed = first_line.contains("[installed]");
                        
                        result.push(PackageInfo {
                            name,
                            version,
                            repo,
                            description,
                            installed,
                        });
                    }
                }
            }
        }
    }

    Ok(result)
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
            get_cache_size,
            install_polkit_policy,
            check_polkit_policy,
            get_popular_packages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
