// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dev_wallet::{
    sqlite::SqliteVault, vault_interface::VaultInterface, wallet::WalletInputBuilder,
};
// use serde_json::{json, Value};
use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

struct AppState {
    wallet: Arc<Mutex<WalletInputBuilder>>,
    vault: Arc<Mutex<SqliteVault>>,
}

#[tauri::command]
async fn generate_mnemonic(state: State<'_, AppState>) -> Result<String, String> {
    let mut wallet = state.wallet.lock().await;

    wallet.regenerate_mnemonic();
    // wallet
    Ok(wallet.mnemonic_as_string())
}

// #[tauri::command]
// async fn authenticate(
//     name: String,
//     password: String,
//     state: State<'_, AppState>,
// ) -> Result<Value, String> {
//     Ok(json!({}))
// }

#[tauri::command]
async fn create_wallet(
    name: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut wallet = state.wallet.lock().await;
    let vault = state.vault.lock().await;
    wallet.name(&name);
    wallet.password(&password);
    let wallet = wallet.build().unwrap();
    let result = vault.insert_wallet(wallet).await;

    if let Err(err) = result {
        return Err(err.to_string());
    }

    Ok("success".to_string())
}

// #[tauri::command]
// async fn create_account(
//     path: String,
//     wallet_id: String,
//     password: String,
//     state: State<'_, AppState>,
// ) -> Result<Value, String> {
//     Ok(json!({}))
// }

#[async_std::main]
async fn main() {
    let vault = SqliteVault::new(Some("sqlite://database.db")).await;
    vault.migrate().await;

    let app_state = AppState {
        wallet: Arc::new(Mutex::new(WalletInputBuilder::new())),
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
            // authenticate,
            // create_account
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
