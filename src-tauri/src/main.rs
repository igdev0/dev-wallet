// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bitcoin_wallet::{
    storage::{self, DbFacade, DbFacadePool},
    wallet::WalletBuilder,
};
use sqlx::{Pool, Sqlite, SqliteConnection};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

struct AppState {
    mnemonics: Arc<Mutex<String>>,
    db_pool: Arc<Mutex<DbFacadePool>>,
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

#[tauri::command]
fn create_wallet(input: String, state: State<'_, AppState>) -> String {
    println!("{}", &input);
    let mnemonics = state.mnemonics.lock().unwrap();
    let mut wallet_builder = WalletBuilder::new(&*mnemonics);
    wallet_builder.passphrase(input.as_str());
    let wallet = wallet_builder.build();

    "Wallet created".to_string()
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
