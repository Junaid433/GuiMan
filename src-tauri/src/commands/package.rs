use crate::models::{PackageInfo, CommandResult};
use crate::{pacman, aur, utils};
use tauri::Window;

#[tauri::command]
pub async fn list_installed() -> Result<Vec<PackageInfo>, String> {
    pacman::list_installed_packages()
}

#[tauri::command]
pub async fn check_updates() -> Result<Vec<PackageInfo>, String> {
    pacman::check_for_updates()
}

#[tauri::command]
pub async fn list_orphans() -> Result<Vec<String>, String> {
    pacman::list_orphan_packages()
}

#[tauri::command]
pub async fn get_package_history() -> Result<Vec<PackageInfo>, String> {
    pacman::get_package_history()
}

#[tauri::command]
pub async fn get_package_info(
    pkg: String,
    repo: Option<String>,
    is_installed: Option<bool>,
) -> Result<serde_json::Value, String> {
    let repo = repo.unwrap_or_else(|| "unknown".to_string());
    let is_installed = is_installed.unwrap_or(false);

    let stdout = if repo == "aur" && !is_installed {
        aur::get_aur_package_info(&pkg, "yay")?
    } else {
        pacman::get_package_info(&pkg, &repo, is_installed)?
    };

    let info = utils::parse_package_info(&stdout);
    Ok(serde_json::Value::Object(info))
}

#[tauri::command]
pub async fn list_aur_packages(helper: String) -> Result<Vec<PackageInfo>, String> {
    aur::list_aur_packages(&helper)
}

#[tauri::command]
pub async fn install_package(window: Window, pkg: String) -> Result<CommandResult, String> {
    pacman::install_package_async(window, pkg).await
}

#[tauri::command]
pub async fn remove_package(window: Window, pkg: String) -> Result<CommandResult, String> {
    pacman::remove_package_async(window, pkg).await
}

// AUR Advanced Features

#[tauri::command]
pub async fn vote_aur_package(package: String, helper: String) -> Result<String, String> {
    aur::vote_aur_package(&package, &helper)
}

#[tauri::command]
pub async fn flag_aur_package(package: String, helper: String, comment: Option<String>) -> Result<String, String> {
    aur::flag_aur_package(&package, &helper, comment)
}

#[tauri::command]
pub async fn adopt_aur_package(package: String, helper: String) -> Result<String, String> {
    aur::adopt_aur_package(&package, &helper)
}

#[tauri::command]
pub async fn get_aur_build_info(package: String, helper: String) -> Result<String, String> {
    aur::get_aur_build_info(&package, &helper)
}

#[tauri::command]
pub async fn install_aur_with_options(package: String, helper: String, options: Vec<String>) -> Result<String, String> {
    aur::install_aur_with_options(&package, &helper, options)
}

