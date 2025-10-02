use crate::models::PackageInfo;
use crate::utils::{strip_ansi_codes, is_command_available};
use std::process::Command;
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

/// Search AUR packages using the specified helper
pub fn search_aur(
    query: &str,
    helper: &str,
    matcher: &SkimMatcherV2,
) -> Result<Vec<(i64, PackageInfo)>, String> {
    let helper_cmd = match helper {
        "yay" => "yay",
        "paru" => "paru",
        _ => "yay",
    };

    if !is_command_available(helper_cmd) {
        return Ok(Vec::new());
    }

    let output = Command::new(helper_cmd)
        .args(&["-Ss", query])
        .output()
        .map_err(|e| format!("Failed to search AUR: {}", e))?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    let mut scored_packages = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        if line.contains("aur/") {
            let clean = strip_ansi_codes(line);

            if let Some(aur_pos) = clean.find("aur/") {
                let after_aur = &clean[aur_pos + 4..];
                let parts: Vec<&str> = after_aur.split_whitespace().collect();

                if !parts.is_empty() {
                    let name = parts[0];
                    let version = if parts.len() > 1 {
                        parts[1].to_string()
                    } else {
                        "unknown".to_string()
                    };
                    let installed = line.contains("[installed]");

                    if let Some(score) = matcher.fuzzy_match(name, query) {
                        let description = if i + 1 < lines.len() {
                            let next_line = lines[i + 1].trim();
                            if !next_line.contains("aur/") && !next_line.is_empty() {
                                next_line.to_string()
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        };

                        scored_packages.push((
                            score,
                            PackageInfo {
                                name: name.to_string(),
                                version,
                                repo: "aur".to_string(),
                                description,
                                installed,
                            },
                        ));
                    }
                }
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    Ok(scored_packages)
}

