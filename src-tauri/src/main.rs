// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::path::PathBuf;
use std::fs;

#[tauri::command]
fn get_files_in_folder(folder_path: String) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let entries = fs::read_dir(folder_path).expect("Failed to read directory");

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                files.push(path.to_owned());
            }
        }
    }

    files
}

#[tauri::command]
fn open_dialog() -> Option<String> {
    // tauri::api::dialog::open_directory().ok()
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
			open_dialog,
			get_files_in_folder
		])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
