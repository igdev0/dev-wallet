use bip39::Mnemonic;
use bitcoin_wallet::{
    storage::{self, StorageMethods},
    wallet::WalletBuilder,
};
use rand::RngCore;
use rand_core::{self, OsRng};
use sqlx::{Pool, Sqlite};

#[tokio::test]
async fn create_wallet() {
    let mut connection = storage::DbFacade::new(Some("sqlite://database.db")).await;
    connection.migrate().await;
    let db = connection.pool;
    let mut entropy = [0u8; 32];
    let mut rng = OsRng;
    rng.fill_bytes(&mut entropy);
    let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();

    let mut wallet = WalletBuilder::new(&mnemonic.to_string());
    wallet.name("Main wallet".to_string());

    let wallet = wallet.build();

    wallet.save(&db).await;

    wallet.load(&db).await;
}
