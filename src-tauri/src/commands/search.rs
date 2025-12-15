use crate::aur;
use crate::models::PackageInfo;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::process::Command;

#[tauri::command]
pub async fn search_package(
    query: String,
    aur_enabled: Option<bool>,
    aur_helper: Option<String>,
) -> Result<Vec<PackageInfo>, String> {
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    let matcher = SkimMatcherV2::default();
    let mut scored_packages: Vec<(i64, PackageInfo)> = Vec::new();

    // Search official repositories
    let output = Command::new("/usr/bin/pacman")
        .args(["-Sl"])
        .output()
        .map_err(|e| format!("Failed to execute pacman: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let repo = parts[0];
            let name = parts[1];
            let version = parts[2];
            let installed = line.contains("[installed]");

            if let Some(score) = matcher.fuzzy_match(name, &query) {
                scored_packages.push((
                    score,
                    PackageInfo {
                        name: name.to_string(),
                        version: version.to_string(),
                        repo: repo.to_string(),
                        description: String::new(),
                        installed,
                    },
                ));
            }
        }
    }

    // Search AUR if enabled
    if aur_enabled.unwrap_or(false) {
        let helper = aur_helper.unwrap_or_else(|| "yay".to_string());
        if let Ok(aur_results) = aur::search_aur(&query, &helper, &matcher) {
            scored_packages.extend(aur_results);
        }
    }

    scored_packages.sort_by(|a, b| b.0.cmp(&a.0));
    scored_packages.truncate(50);

    let mut packages: Vec<PackageInfo> = scored_packages.into_iter().map(|(_, pkg)| pkg).collect();

    // Get descriptions for official packages only
    enrich_package_descriptions(&mut packages)?;

    Ok(packages)
}

/// Enrich packages with descriptions from pacman
fn enrich_package_descriptions(packages: &mut [PackageInfo]) -> Result<(), String> {
    let official_packages: Vec<String> = packages
        .iter()
        .filter(|p| p.repo != "aur")
        .map(|p| p.name.clone())
        .collect();

    if official_packages.is_empty() {
        return Ok(());
    }

    let info_output = Command::new("/usr/bin/pacman")
        .args(["-Si"])
        .args(&official_packages)
        .output()
        .ok();

    if let Some(info) = info_output {
        let info_str = String::from_utf8_lossy(&info.stdout);
        let mut current_name = String::new();
        let mut current_desc = String::new();

        for line in info_str.lines() {
            if line.starts_with("Name") {
                if !current_name.is_empty() && !current_desc.is_empty() {
                    if let Some(pkg) = packages
                        .iter_mut()
                        .find(|p| p.name == current_name && p.repo != "aur")
                    {
                        pkg.description = current_desc.clone();
                    }
                }
                current_name = line.split(':').nth(1).unwrap_or("").trim().to_string();
                current_desc = String::new();
            } else if line.starts_with("Description") {
                current_desc = line.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }

        if !current_name.is_empty() && !current_desc.is_empty() {
            if let Some(pkg) = packages
                .iter_mut()
                .find(|p| p.name == current_name && p.repo != "aur")
            {
                pkg.description = current_desc;
            }
        }
    }

    Ok(())
}
