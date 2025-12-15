#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Module declarations
mod aur;
mod commands;
mod error;
mod models;
mod pacman;
mod utils;

use commands::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Search commands
            search_package,
            get_popular_packages,
            // Package commands
            list_installed,
            get_package_counts,
            check_updates,
            list_orphans,
            get_package_history,
            get_package_info,
            list_aur_packages,
            install_package,
            remove_package,
            // AUR Advanced Features
            vote_aur_package,
            flag_aur_package,
            adopt_aur_package,
            get_aur_build_info,
            install_aur_with_options,
            // System commands
            update_system,
            clean_cache,
            export_package_list,
            get_cache_size,
            check_polkit_policy,
            install_polkit_policy,
            // Groups commands
            list_groups,
            get_group_packages,
            install_group,
            // Files commands
            list_package_files,
            find_file_owner,
            search_files,
            list_package_backups,
            // Repository commands
            list_repositories,
            get_repo_packages,
            sync_databases,
            get_mirrorlist_info,
            update_mirrorlist,
            rank_mirrors,
            // Dependency commands
            get_dependency_tree,
            get_reverse_dependency_tree,
            // Backup commands
            create_package_backup,
            list_backups,
            restore_package_backup,
            delete_package_backup,
            list_pacman_hooks,
            export_packages,
            // Updater commands
            check_app_updates,
            install_app_update,
            restart_application,
            is_appimage,
            get_update_channel,
            set_auto_update,
            get_auto_update_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
