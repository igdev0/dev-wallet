use bip39::Mnemonic;
use bitcoin::hex::{Case, DisplayHex};
use bitcoin_wallet::{
    account::AccountBuilder,
    path_builder::PathBuilder,
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
    let wallet = wallet.build();
}

#[test]

fn can_build_bip32_path() {
    let path = PathBuilder::new();
    let path = path.build().to_string();

    assert_eq!(path, "49'/0'/0'/0/0");
}
#[test]
fn can_build_account() {
    // Account
    let mnemonic = mnemonic_helper();
    let seed = mnemonic.to_seed("passphrase");
    let mut account_builder = AccountBuilder::new();

    account_builder.seed(&seed.to_hex_string(Case::Lower));

    let account_result = account_builder.build();

    if let Ok(account) = account_result {
        assert!(account.address.len() > 0);
    }
}
