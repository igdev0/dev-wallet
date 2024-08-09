// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Mutex;
use tauri::{Manager, State};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(Default)]
struct AppState {
    mnemonics: String,
}

#[tauri::command]
fn generate_mnemonic(state: State<'_, Mutex<AppState>>) -> String {
    let mnemonics: String = bitcoin_wallet::utils::generate_mnemonic()
        .unwrap()
        .to_string();
    let mut state = state.lock().unwrap();
    state.mnemonics = mnemonics;
    state.mnemonics.clone()
}

#[async_std::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![generate_mnemonic])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
