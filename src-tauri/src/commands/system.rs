use crate::models::CommandResult;
use crate::pacman;
use tauri::Window;
use std::process::Command;
use std::path::Path;

#[tauri::command]
pub async fn update_system(window: Window) -> Result<CommandResult, String> {
    pacman::update_system_async(window).await
}

#[tauri::command]
pub async fn clean_cache(window: Window, aur_helper: Option<String>) -> Result<CommandResult, String> {
    pacman::operations::clean_cache_async(window, aur_helper).await
}

#[tauri::command]
pub async fn export_package_list() -> Result<Vec<String>, String> {
    pacman::export_package_list()
}

#[tauri::command]
pub async fn get_cache_size() -> Result<serde_json::Value, String> {
    let pacman_size = pacman::get_cache_size()?;
    
    // Check for yay cache
    let yay_cache_path = std::path::Path::new(&std::env::var("HOME").unwrap_or_default())
        .join(".cache/yay");
    
    let yay_size = if yay_cache_path.exists() {
        let output = Command::new("/usr/bin/du")
            .args(&["-sh", yay_cache_path.to_str().unwrap()])
            .output()
            .ok();
        
        if let Some(out) = output {
            String::from_utf8_lossy(&out.stdout)
                .split_whitespace()
                .next()
                .unwrap_or("0")
                .to_string()
        } else {
            "0".to_string()
        }
    } else {
        "0".to_string()
    };
    
    Ok(serde_json::json!({
        "pacman": pacman_size,
        "yay": yay_size,
        "yay_path": yay_cache_path.to_str().unwrap_or("~/.cache/yay")
    }))
}

#[tauri::command]
pub async fn get_popular_packages() -> Result<Vec<crate::models::PackageInfo>, String> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::process::Command;

    let popular_packages = vec![
        "firefox", "chromium", "vlc", "gimp", "libreoffice-fresh", "thunderbird",
        "inkscape", "blender", "audacity", "obs-studio", "kdenlive", "krita",
        "telegram-desktop", "discord", "spotify-launcher", "steam", "lutris",
        "wine", "docker", "virtualbox", "qemu", "libvirt", "ansible", "terraform",
        "kubectl", "helm", "git", "vim", "neovim", "emacs", "vscode", "code",
        "nodejs", "npm", "python", "python-pip", "rust", "go", "ruby", "php",
        "mariadb", "postgresql", "redis", "mongodb", "nginx", "apache", "caddy",
        "htop", "tmux", "zsh", "fish", "starship", "alacritty", "kitty",
        "bat", "exa", "fd", "ripgrep", "fzf", "jq", "yq", "wget", "curl",
        "rsync", "rclone", "syncthing", "nextcloud-client", "keepassxc",
        "pass", "bitwarden", "veracrypt", "gnupg", "openssh", "wireguard-tools",
        "openvpn", "nmap", "wireshark-qt", "tcpdump", "netcat", "socat",
        "ffmpeg", "imagemagick", "graphicsmagick", "pandoc", "hugo", "jekyll",
        "latex", "texlive-core", "texlive-latexextra", "r", "jupyter-notebook",
        "octave", "scilab", "maxima", "sagemath", "gcc", "clang", "cmake",
        "make", "autoconf", "automake", "gdb", "valgrind", "strace", "ltrace",
        "doxygen", "sphinx", "gtk3", "gtk4", "qt5-base", "qt6-base", "sdl2",
        "vulkan-tools", "mesa", "lib32-mesa", "nvidia", "nvidia-utils",
        "xorg-server", "wayland", "sway", "i3-wm", "awesome", "bspwm",
        "plasma-desktop", "gnome", "xfce4", "mate", "cinnamon", "budgie-desktop",
        "lightdm", "sddm", "gdm", "greetd", "ly", "cups", "bluez", "bluez-utils",
        "pulseaudio", "pipewire", "pipewire-pulse", "wireplumber", "alsa-utils",
        "pavucontrol", "helvum", "gparted", "gnome-disk-utility", "timeshift",
        "borgbackup", "restic", "duplicati", "baobab", "ncdu", "dust",
        "neofetch", "screenfetch", "fastfetch", "btop", "bottom", "glances",
        "nmon", "iotop", "nethogs", "iftop", "bmon", "vnstat", "speedtest-cli",
        "yt-dlp", "youtube-dl", "mpv", "mplayer", "smplayer", "celluloid",
        "handbrake", "makemkv", "kdenlive", "shotcut", "flowblade", "openshot",
        "darktable", "rawtherapee", "digikam", "gwenview", "gthumb", "shotwell",
        "calibre", "foliate", "zathura", "mupdf", "evince", "okular", "atril",
    ];

    let mut rng = thread_rng();
    let mut shuffled = popular_packages.clone();
    shuffled.shuffle(&mut rng);

    let selected: Vec<&str> = shuffled.iter().take(20).copied().collect();

    let mut result = Vec::new();

    for pkg_name in selected {
        let output = Command::new("/usr/bin/pacman")
            .args(&["-Ss", &format!("^{}$", pkg_name)])
            .output()
            .ok();

        if let Some(out) = output {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let lines: Vec<&str> = stdout.lines().collect();

            if lines.len() >= 2 {
                let first_line = lines[0];
                let parts: Vec<&str> = first_line.split_whitespace().collect();

                if parts.len() >= 2 {
                    let name_version: Vec<&str> = parts[0].split('/').collect();
                    if name_version.len() == 2 {
                        let description = if lines.len() > 1 {
                            lines[1].trim().to_string()
                        } else {
                            String::new()
                        };

                        result.push(crate::models::PackageInfo {
                            name: name_version[1].to_string(),
                            version: parts[1].to_string(),
                            repo: name_version[0].to_string(),
                            description,
                            installed: first_line.contains("[installed]"),
                        });
                    }
                }
            }
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn check_polkit_policy() -> Result<bool, String> {
    let policy_path = "/usr/share/polkit-1/actions/com.guiman.pkexec.policy";
    Ok(Path::new(policy_path).exists())
}

#[tauri::command]
pub async fn install_polkit_policy() -> Result<CommandResult, String> {
    // Get the path to the polkit policy file in the app resources
    let policy_source = if Path::new("polkit/com.guiman.pkexec.policy").exists() {
        "polkit/com.guiman.pkexec.policy"
    } else if Path::new("../polkit/com.guiman.pkexec.policy").exists() {
        "../polkit/com.guiman.pkexec.policy"
    } else {
        return Err("Polkit policy file not found".to_string());
    };

    let output = Command::new("/usr/bin/pkexec")
        .args(&["cp", policy_source, "/usr/share/polkit-1/actions/"])
        .output()
        .map_err(|e| format!("Failed to install polkit policy: {}", e))?;

    if output.status.success() {
        Ok(CommandResult::success(
            "Polkit policy installed successfully! Password-free package management is now enabled.".to_string()
        ))
    } else {
        Err(format!(
            "Failed to install polkit policy: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

