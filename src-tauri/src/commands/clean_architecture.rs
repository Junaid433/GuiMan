/// Clean architecture Tauri commands
/// 
/// These commands demonstrate the proper separation of concerns:
/// - Controllers handle orchestration between services and GUI
/// - Services contain pure business logic
/// - GUI events are handled through the event emitter trait

use crate::controllers::*;
use crate::models::CommandResult;
use tauri::Window;

/// Install a package using clean architecture
#[tauri::command]
pub async fn install_package_clean(window: Window, package: String) -> Result<CommandResult, String> {
    PackageController::install_package_async(window, package).await
}

/// Remove a package using clean architecture
#[tauri::command]
pub async fn remove_package_clean(window: Window, package: String) -> Result<CommandResult, String> {
    PackageController::remove_package_async(window, package).await
}

/// Update the system using clean architecture
#[tauri::command]
pub async fn update_system_clean(window: Window) -> Result<CommandResult, String> {
    PackageController::update_system_async(window).await
}

/// Clean cache using clean architecture
#[tauri::command]
pub async fn clean_cache_clean(window: Window, helper_cmd: Option<String>) -> Result<CommandResult, String> {
    CacheController::clean_cache_async(window, helper_cmd).await
}

/// Get cache size using clean architecture
#[tauri::command]
pub fn get_cache_size_clean() -> Result<String, String> {
    SystemController::get_cache_size()
}

/// Check polkit policy using clean architecture
#[tauri::command]
pub fn check_polkit_policy_clean() -> Result<bool, String> {
    SystemController::check_polkit_policy()
}

/// Install polkit policy using clean architecture
#[tauri::command]
pub fn install_polkit_policy_clean(window: Window) -> Result<CommandResult, String> {
    SystemController::install_polkit_policy_async(window)
}
