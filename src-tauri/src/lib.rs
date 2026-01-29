pub mod db;
pub mod reports;

use std::fs;
use std::sync::Mutex;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_app_info(state: tauri::State<db::AppState>) -> String {
    let db = state.db.lock().unwrap();
    match *db {
        Some(_) => "Database Connected".to_string(),
        None => "Database Not Connected".to_string(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().expect("failed to get app data dir");

            // Ensure directory exists
            if !app_data_dir.exists() {
                fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
            }

            let db_path = app_data_dir.join("finance.db");

            let conn = db::init_db(&db_path).expect("failed to initialize database");

            app.manage(db::AppState {
                db: Mutex::new(Some(conn)),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_app_info, reports::get_trial_balance])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
