// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bitcoin::bip32::DerivationPath;
use dev_wallet::{
    account::AccountInputBuilder, sqlite::SqliteVault, vault_interface::VaultInterface,
    wallet::WalletInputBuilder,
};
use serde_json::{json, Value};
use std::{str::FromStr, sync::Arc};
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
    Ok(wallet.mnemonic_as_string())
}

#[tauri::command]
async fn authenticate(
    name: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let vault = state.vault.lock().await;
    let wallet = vault.get_wallet_by_name(&name).await;
    if let Err(err) = wallet {
        return Err(err.to_string());
    }
    let wallet = wallet.unwrap();
    let key = wallet.authenticate(&password);

    if let Err(err) = key {
        return Err(err.to_string());
    }

    Ok(wallet.to_json())
}

#[tauri::command]
async fn create_wallet(
    name: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let mut wallet = state.wallet.lock().await;
    let vault = state.vault.lock().await;
    wallet.name(&name);
    wallet.password(&password);
    let wallet = wallet.build().unwrap();
    let result = vault.insert_wallet(wallet).await;

    if let Err(err) = result {
        return Err(err.to_string());
    }

    Ok(result.unwrap().to_json())
}

#[tauri::command]
async fn create_account(
    path: String,
    wallet_id: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let vault = state.vault.lock().await;
    let wallet = vault.get_wallet_by_id(&wallet_id).await;

    if let Err(err) = wallet {
        return Err(err.to_string());
    }

    let wallet = wallet.unwrap();
    let key = wallet.authenticate(&password);

    if let Err(err) = key {
        return Err(err.to_string());
    }

    let path = DerivationPath::from_str(&path);

    if let Err(err) = path {
        return Err(err.to_string());
    }

    let mut account = AccountInputBuilder::from(wallet);
    account.path(path.unwrap());

    let account = account.build(key.unwrap());

    if let Err(err) = account {
        return Err(err.to_string());
    }

    let account = vault.insert_account(account.unwrap()).await;

    if let Err(err) = account {
        return Err(err.to_string());
    }

    Ok(account.unwrap().to_json())
}

#[tauri::command]
async fn remove_wallet(
    id: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let vault = state.vault.lock().await;
    let wallet = vault.get_wallet_by_id(&id).await;
    if let Err(err) = wallet {
        return Err(err.to_string());
    }
    let auth_res = wallet.unwrap().authenticate(&password);

    if let Err(err) = auth_res {
        return Err(err.to_string());
    }

    let res = vault.remove_wallet_by_id(&id).await;
    if let Err(err) = res {
        return Err(err.to_string());
    }
    Ok(json!({"success": true}))
}

#[tauri::command]
async fn remove_account(
    id: String,
    wallet_id: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<Value, String> {
    let vault = state.vault.lock().await;
    let wallet = vault.get_wallet_by_id(&wallet_id).await;
    if let Err(err) = wallet {
        return Err(err.to_string());
    }
    let auth_res = wallet.unwrap().authenticate(&password);

    if let Err(err) = auth_res {
        return Err(err.to_string());
    }

    let res = vault.remove_account_by_id(&id).await;
    if let Err(err) = res {
        return Err(err.to_string());
    }
    Ok(json!({"success": true}))
}

#[tauri::command]
async fn list_accounts(wallet_id: String, state: State<'_, AppState>) -> Result<Value, String> {
    let vault = state.vault.lock().await;
    let accounts = vault.get_all_accounts(&wallet_id).await;
    if let Err(err) = accounts {
        return Err(err.to_string());
    }

    Ok(accounts
        .unwrap()
        .iter()
        .map(|item| item.to_json())
        .collect())
}

#[tauri::command]
async fn list_wallets(state: State<'_, AppState>) -> Result<Value, String> {
    let vault = state.vault.lock().await;
    let wallets = vault.get_all_wallets().await;
    if let Err(err) = wallets {
        return Err(err.to_string());
    }

    Ok(wallets.unwrap().iter().map(|item| item.to_json()).collect())
}

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
            authenticate,
            create_account,
            remove_wallet,
            remove_account,
            list_accounts,
            list_wallets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
