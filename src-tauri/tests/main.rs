use dev_wallet::*;
use tokio;
use vault::{interface::VaultInterface, sqlite::SqliteVault, wallet::WalletInputBuilder};

#[tokio::test]
async fn can_create_wallet() {
    let mut wallet = WalletInputBuilder::new();
    wallet.name("name");
    wallet.password("password");
    let wallet = wallet.build();
    let name = wallet.name.clone();
    let vault = SqliteVault::new(Some("sqlite::memory:")).await;

    vault.insert_wallet(wallet).await.unwrap();

    let result_wallet = vault.get_wallet_by_name(&name).await.unwrap();
    assert_eq!(result_wallet.name, name);
}
src/vault/sqlite.rs