use dev_wallet::*;
use tokio;
use vault::{
    account::AccountInputBuilder, interface::VaultInterface, sqlite::SqliteVault,
    wallet::WalletInputBuilder,
};

#[tokio::test]
async fn can_create_wallet() {
    let mut wallet = WalletInputBuilder::new();

    wallet.name("name");
    wallet.password("password");

    let wallet = wallet.build().unwrap();
    let name = wallet.name.clone();
    let vault = SqliteVault::new(Some("sqlite::memory:")).await;
    vault.migrate().await;
    vault.insert_wallet(wallet).await.unwrap();

    let result_wallet = vault.get_wallet_by_name(&name).await.unwrap();
    assert_eq!(result_wallet.name, name);
}

#[tokio::test]
async fn can_create_account() {
    let name = "main";
    let password = "password123";
    let mut wallet = WalletInputBuilder::new();

    wallet.name(&name);
    wallet.password(&password);

    let wallet = wallet.build().unwrap();
    let name = wallet.name.clone();
    let vault = SqliteVault::new(Some("sqlite::memory:")).await;
    vault.migrate().await;
    vault.insert_wallet(wallet).await.unwrap();

    let result_wallet = vault.get_wallet_by_name(&name).await.unwrap();

    let key = result_wallet.authenticate(password).unwrap();
    let account = AccountInputBuilder::from(result_wallet);

    let account = account.build(key).unwrap();
    let account = vault.insert_account(account).await.unwrap();

    let account_retrieved = vault.get_account_by_id(&account.id).await.unwrap();

    assert_eq!(account_retrieved.id, account.id);
}
