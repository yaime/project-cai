pub mod db;
pub mod auth;
pub mod ocr;

use std::fs;
use std::sync::Mutex;
use tauri::Manager;

struct OcrState {
    service: Mutex<Option<ocr::OcrService>>,
}

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

#[tauri::command]
async fn perform_ocr(state: tauri::State<'_, OcrState>, path: String) -> Result<ocr::TextResult, String> {
    let mut service = state.service.lock().unwrap();
    if let Some(svc) = service.as_mut() {
        svc.infer(&path).map_err(|e| e.to_string())
    } else {
        Err("OCR Service not available (Model missing or failed to load)".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
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

            // Initialize OCR Service
            // Try to find the model in resources directory
            let model_path = std::env::current_dir()
                .unwrap_or_default()
                .join("resources/ch_PP-OCRv4_rec_infer.onnx");
            let keys_path = std::env::current_dir()
                .unwrap_or_default()
                .join("resources/ppocr_keys_v1.txt");

            let ocr_service = ocr::OcrService::new(
                model_path.to_str().unwrap_or(""),
                keys_path.to_str().unwrap_or("")
            ).ok();

            if ocr_service.is_none() {
                println!("OCR Model not loaded. Expected at: {:?}", model_path);
            }

            app.manage(OcrState {
                service: Mutex::new(ocr_service),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_app_info, perform_ocr, auth::login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
