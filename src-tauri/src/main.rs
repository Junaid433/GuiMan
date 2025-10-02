#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Module declarations
mod models;
mod utils;
mod pacman;
mod aur;
mod commands;

use commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Search commands
            search_package,
            get_popular_packages,
            
            // Package commands
            list_installed,
            check_updates,
            list_orphans,
            get_package_history,
            get_package_info,
            list_aur_packages,
            install_package,
            remove_package,
            
            // System commands
            update_system,
            clean_cache,
            export_package_list,
            get_cache_size,
            check_polkit_policy,
            install_polkit_policy,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
