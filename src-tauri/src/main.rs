// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bitcoin::bip32::DerivationPath;
use dev_wallet::{
    storage::{self, DbFacadePool},
    wallet::WalletBuilder,
};
use serde_json::{json, Value};
use std::{str::FromStr, sync::Arc};
use tauri::{Manager, State};
use tokio::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

struct AppState {
    mnemonics: Arc<Mutex<String>>,
    db_pool: Arc<Mutex<DbFacadePool>>,
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
    let db = &state.db_pool.lock().await;
    let mut wallet = WalletBuilder::from_existing(&name);
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

#[tauri::command]
async fn create_account(
    path: String,
    wallet_id: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let db = &state.db_pool.lock().await;
    let mut wallet_builder = WalletBuilder::from_existing_id(&wallet_id);
    let auth_response = wallet_builder.authenticate(&password, db).await;

    match auth_response {
        Ok(mut wallet) => {
            let mut account_builder = wallet.create_account();
            let path = DerivationPath::from_str(&path);

            if let Err(_) = path {
                return Err("Derivation path incorrect".to_string());
            }
            let path = path.unwrap();
            account_builder.path(path);
            let account = account_builder.build();

            if let Err(e) = account {
                return Err(e.to_string());
            }

            let account = account.unwrap();
            account.save(db).await;

            let wallet = wallet.load_accounts(db).await;
            let accounts_parsed: Vec<Value> =
                wallet.accounts.iter().map(|v| v.parse_as_json()).collect();
            Ok(json!({"accounts": accounts_parsed}))
        }
        Err(err) => return Err(err.to_string()),
    }
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
            authenticate,
            create_account
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
