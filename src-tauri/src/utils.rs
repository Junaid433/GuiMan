use std::process::Command;

/// Strip ANSI escape sequences from a string
pub fn strip_ansi_codes(input: &str) -> String {
    input
        .replace("\x1b[1m", "")
        .replace("\x1b[0m", "")
        .replace("\x1b]8;;", "")
        .replace("\x1b\\", "")
}

/// Check if a command is available in PATH
pub fn is_command_available(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Parse package info from pacman output format
pub fn parse_package_info(stdout: &str) -> serde_json::Map<String, serde_json::Value> {
    let mut info = serde_json::Map::new();
    
    for line in stdout.lines() {
        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim().to_lowercase().replace(' ', "_");
            let value = line[pos + 1..].trim().to_string();
            info.insert(key, serde_json::Value::String(value));
        }
    }
    
    info
}

