use async_std::task::AccessError;
use bip39::Mnemonic;
use bitcoin_wallet::{
    storage::{self},
    wallet::WalletBuilder,
};
use rand::RngCore;
use rand_core::{self, OsRng};

fn mnemonic_helper() -> Mnemonic {
    let mut entropy = [0u8; 32];
    let mut rng = OsRng;
    rng.fill_bytes(&mut entropy);
    let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
    mnemonic
}

#[tokio::test]
async fn create_wallet() {
    let connection = storage::DbFacade::new(Some("sqlite::memory:")).await;
    connection.migrate().await;
    let db = connection.pool;
    let mnemonic = mnemonic_helper();
    let mut wallet = WalletBuilder::new(&mnemonic.to_string());
    wallet.name("Main wallet".to_string());

    let wallet = wallet.build();

    wallet.save(&db).await;
}

#[tokio::test]
async fn load_wallet() {
    let conn_facade = storage::DbFacade::new(Some("sqlite::memory:")).await;
    conn_facade.migrate().await;

    let mnemonic = mnemonic_helper();

    let mut wallet = WalletBuilder::new(&mnemonic.to_string());
    wallet.name("Main wallet".to_string());
    wallet.build().save(&conn_facade.pool).await;

    let wallet = WalletBuilder::from_existing("Main wallet");
    let wallet = wallet.load(&conn_facade.pool).await;

    assert_eq!(wallet.unwrap().name, "Main wallet")
}

#[test]
fn can_create_account() {
    let mnemonic = mnemonic_helper();
    let mut wallet = WalletBuilder::new(&mnemonic.to_string());
    wallet.passphrase(String::from("StrongPassphrase"));
    wallet.build();
}
