// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{App, AppHandle, GlobalShortcutManager, Manager};

const SHORTCUT: &str = "Ctrl+Space";

fn register_shortcut(app: &mut App) -> Result<(), tauri::Error>  {
    let app_handle = app.handle();
    let mut shortcuts = app_handle.global_shortcut_manager();
    // Only register if we haven't already assigned something to
    // this key combo
    if !shortcuts.is_registered(SHORTCUT)? {
        shortcuts.register(
            SHORTCUT,
            move || toggle_launchbar(&app_handle)
        )?;
    }

    Ok(())
}
// This should match the "label" in your `tauri.config.json` file.
const WINDOW: &str = "launcher";

fn toggle_launchbar(app: &AppHandle) {
    let window = app.get_window(WINDOW).unwrap();

    // Check if it is currently visible & hide if so.
    if let Ok(true) = window.is_visible() {
        let _ = window.hide();
    } else {
        let _ = window.show();
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() {
    tauri::Builder::default()
        .setup(|a| Ok(register_shortcut(a)?))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

