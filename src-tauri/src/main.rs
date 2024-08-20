// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bitcoin_wallet::db;
use sqlx::SqliteConnection;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

struct AppState {
    mnemonics: Arc<Mutex<String>>,
    db_pool: Arc<Mutex<SqliteConnection>>,
}

#[tauri::command]
fn generate_mnemonic(state: State<'_, AppState>) -> String {
    let mnemonics: String = bitcoin_wallet::utils::generate_mnemonic()
        .unwrap()
        .to_string();

    let mut m = state.mnemonics.lock().unwrap();
    *m = mnemonics.clone();

    mnemonics
}

#[async_std::main]
async fn main() {
    let db_pool = db::create_db_connection().await.unwrap();
    let app_state = AppState {
        mnemonics: Arc::new(Mutex::new("".to_string())),
        db_pool: Arc::new(Mutex::new(db_pool)),
    };

    tauri::Builder::default()
        .setup(|app| {
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![generate_mnemonic])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
