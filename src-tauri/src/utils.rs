use regex::Regex;
use std::process::Command;

/// Strip ANSI escape sequences from a string
pub fn strip_ansi_codes(input: &str) -> String {
    let pattern = Regex::new(r"\x1b\[[0-9;]*[A-Za-z]|\x1b\][^\x07]*(\x07|\x1b\\)").ok();
    match pattern {
        Some(pattern) => pattern.replace_all(input, "").to_string(),
        None => input.to_string(),
    }
}

/// Check if a command is available in PATH
pub fn is_command_available(cmd: &str) -> bool {
    Command::new("/usr/bin/which")
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
