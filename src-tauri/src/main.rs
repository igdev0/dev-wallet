// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bip32::Prefix;
use bip39::Mnemonic;
use bitcoin_wallet::{db, wallet::WalletBuilder};
use sqlx::SqliteConnection;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};
use tauri::{App, Manager, State};

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

#[tauri::command]
fn create_wallet(input: String, state: State<'_, AppState>) -> String {
    println!("{}", &input);
    let mnemonics = state.mnemonics.lock().unwrap();
    let mut wallet_builder = WalletBuilder::new(&*mnemonics);
    wallet_builder.passphrase(input);
    let wallet = wallet_builder.build();

    "Wallet created".to_string()
}

#[async_std::main]
async fn main() {
    let db_pool = db::create_db_connection().await.unwrap();
    let app_state = AppState {
        mnemonics: Arc::new(Mutex::new(String::from(""))),
        db_pool: Arc::new(Mutex::new(db_pool)),
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
