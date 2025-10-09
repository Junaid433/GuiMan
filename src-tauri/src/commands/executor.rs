use crate::models::CommandResult;
use crate::utils::create_command;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use tokio;

/// Command execution result with output lines
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub output_lines: Vec<String>,
    pub error_lines: Vec<String>,
}

/// Execute a command and return structured results
pub fn execute_command(cmd: &str, args: &[&str]) -> Result<ExecutionResult, String> {
    let output = create_command(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", cmd, e))?;

    let stdout_lines: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();

    let stderr_lines: Vec<String> = String::from_utf8_lossy(&output.stderr)
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(ExecutionResult {
        success: output.status.success(),
        output_lines: stdout_lines,
        error_lines: stderr_lines,
    })
}

/// Execute a command with real-time output streaming
pub async fn execute_command_streaming<F>(
    cmd: &str,
    args: &[&str],
    mut on_output: F,
) -> Result<ExecutionResult, String>
where
    F: FnMut(String) + Send + 'static,
{
    let mut child = create_command(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start {}: {}", cmd, e))?;

    let mut output_lines = Vec::new();
    let mut error_lines = Vec::new();

    // Stream stdout
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                on_output(line.clone());
                output_lines.push(line);
            }
        }
    }

    // Stream stderr
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                on_output(line.clone()); // Also stream stderr to the callback
                error_lines.push(line);
            }
        }
    }

    let status = child.wait().map_err(|e| format!("Failed to wait for {}: {}", cmd, e))?;

    Ok(ExecutionResult {
        success: status.success(),
        output_lines,
        error_lines,
    })
}

/// Install a package (business logic only)
pub async fn install_package(package: &str) -> Result<ExecutionResult, String> {
    execute_command_streaming(
        "/usr/bin/pkexec",
        &["/usr/bin/pacman", "-S", "--needed", "--noconfirm", package],
        |_| {}, // No GUI callback in business logic
    ).await
}

/// Remove a package (business logic only)
pub async fn remove_package(package: &str) -> Result<ExecutionResult, String> {
    execute_command_streaming(
        "/usr/bin/pkexec",
        &["/usr/bin/pacman", "-Rs", "--noconfirm", package],
        |_| {}, // No GUI callback in business logic
    ).await
}

/// Update the system (business logic only)
pub async fn update_system() -> Result<ExecutionResult, String> {
    execute_command_streaming(
        "/usr/bin/pkexec",
        &["/usr/bin/pacman", "-Syu", "--needed", "--noconfirm"],
        |_| {}, // No GUI callback in business logic
    ).await
}

/// Clean package cache (business logic only)
pub async fn clean_cache(aur_helper: Option<&str>) -> Result<ExecutionResult, String> {
    let helper = aur_helper.unwrap_or("yay");
    
    // Clean pacman cache
    let pacman_result = execute_command_streaming(
        "/usr/bin/pkexec",
        &["/usr/bin/pacman", "-Scc", "--noconfirm"],
        |_| {},
    ).await?;

    // Clean AUR cache (only if helper is valid)
    let aur_result = if helper == "yay" || helper == "paru" {
        execute_command_streaming(
            "/usr/bin/pkexec",
            &[&format!("/usr/bin/{}", helper), "-Scc"],
            |_| {},
        ).await
    } else {
        Ok(ExecutionResult {
            success: false,
            output_lines: vec!["Invalid AUR helper specified".to_string()],
            error_lines: vec![],
        })
    };

    // Combine results
    let mut combined_output = pacman_result.output_lines;
    if let Ok(aur_result) = aur_result {
        combined_output.extend(aur_result.output_lines);
    }

    Ok(ExecutionResult {
        success: pacman_result.success,
        output_lines: combined_output,
        error_lines: pacman_result.error_lines,
    })
}
