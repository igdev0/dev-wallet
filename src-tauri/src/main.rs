// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bitcoin_wallet::{
    storage::{self, DbFacadePool},
    wallet::WalletBuilder,
};
use serde_json::json;
use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

struct AppState {
    mnemonics: Arc<Mutex<String>>,
    db_pool: Arc<Mutex<DbFacadePool>>,
}

#[tauri::command]
async fn generate_mnemonic(state: State<'_, AppState>) -> Result<String, String> {
    let mnemonics: String = bitcoin_wallet::utils::generate_mnemonic()
        .unwrap()
        .to_string();

    let mut m = state.mnemonics.lock().await;
    *m = mnemonics.clone();

    Ok(mnemonics)
}

#[tauri::command]
async fn create_wallet(
    name: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mnemonics = state.mnemonics.lock().await;
    let db = &state.db_pool.lock().await;
    let mut wallet_builder = WalletBuilder::new(&*mnemonics);
    wallet_builder.passphrase(&password);
    wallet_builder.name(&name);

    let wallet = wallet_builder.build();
    wallet.save(&db).await;
    let result = json!({
        name: wallet.name
    });
    Ok(result.to_string())
    // wallet.name
}

#[async_std::main]
async fn main() {
    let db_pool = storage::DbFacade::new(None).await;
    db_pool.migrate().await;

    let app_state = AppState {
        mnemonics: Arc::new(Mutex::new(String::from(""))),
        db_pool: Arc::new(Mutex::new(db_pool.pool)),
    };

    tauri::Builder::default()
        .setup(|app| {
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![generate_mnemonic, create_wallet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
