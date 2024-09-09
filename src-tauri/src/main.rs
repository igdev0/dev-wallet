// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dev_wallet::vault::sqlite::SqliteVault;
use serde_json::{json, Value};
use std::{str::FromStr, sync::Arc};
use tauri::{Manager, State};
use tokio::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

struct AppState {
    mnemonics: Arc<Mutex<String>>,
    vault: Arc<Mutex<SqliteVault>>,
}

#[tauri::command]
async fn generate_mnemonic(state: State<'_, AppState>) -> Result<String, String> {
    let mnemonics: String = dev_wallet::utils::generate_mnemonic().unwrap().to_string();

    let mut m = state.mnemonics.lock().await;
    *m = mnemonics.clone();

    Ok(mnemonics)
}

#[tauri::command]
async fn authenticate(
    name: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    Ok(json!({}))
}

#[tauri::command]
async fn create_wallet(
    name: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mnemonics = state.mnemonics.lock().await;
    Ok("".to_string())
}

#[tauri::command]
async fn create_account(
    path: String,
    wallet_id: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    Ok(json!({}))
}

#[async_std::main]
async fn main() {
    let vault = SqliteVault::new(Some("sqlite://database.db")).await;
    vault.migrate().await;

    let app_state = AppState {
        mnemonics: Arc::new(Mutex::new(String::from(""))),
        vault: Arc::new(Mutex::new(vault)),
    };

    tauri::Builder::default()
        .setup(|app| {
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            generate_mnemonic,
            create_wallet,
            authenticate,
            create_account
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
