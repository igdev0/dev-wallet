// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bitcoin_wallet::{
    storage::{self, DbFacadePool},
    wallet::{Wallet, WalletBuilder},
    WalletError,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tauri::{App, Manager, State};
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
async fn authenticate(
    name: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let db = &state.db_pool.lock().await;
    let wallet = WalletBuilder::from_existing(&name);
    let auth_result = wallet.authenticate(&password, db).await;

    if let Ok(wallet) = auth_result {
        let value = wallet.serialize_res();
        return Ok(value);
    } else {
        let err = auth_result.err().unwrap();
        Err(err.serialize())
    }
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
    let result = wallet.save(&db).await;

    if let Err(err) = result {
        return Err(err.to_string());
    }

    let parsed = json!({
        "name": wallet.name
    });
    Ok(parsed.to_string())
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
        .invoke_handler(tauri::generate_handler![
            generate_mnemonic,
            create_wallet,
            authenticate
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
