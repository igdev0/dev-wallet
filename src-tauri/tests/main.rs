use dev_wallet::*;
use path_builder::PathBuilder;
use tokio;
use {
    account::AccountInputBuilder, sqlite::SqliteVault, vault_interface::VaultInterface,
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
    vault.migrate().await.unwrap();
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
    vault.migrate().await.unwrap();
    vault.insert_wallet(wallet).await.unwrap();

    let result_wallet = vault.get_wallet_by_name(&name).await.unwrap();

    let key = result_wallet.authenticate(password).unwrap();
    let account = AccountInputBuilder::from(result_wallet);

    let account = account.build(key).unwrap();
    let account = vault.insert_account(account).await.unwrap();

    let account_retrieved = vault.get_account_by_id(&account.id).await.unwrap();

    assert_eq!(account_retrieved.id, account.id);
}

#[tokio::test]

async fn can_find_wallet() {
    let vault = SqliteVault::new(Some("sqlite::memory:")).await;
    vault.migrate().await.unwrap();

    let name = "main";
    let password = "password123";
    let mut wallet = WalletInputBuilder::new();

    wallet.name(&name);
    wallet.password(&password);
    let wallet = wallet.build().unwrap();
    let wallet = vault.insert_wallet(wallet).await.unwrap();

    let wallet = vault.get_wallet_by_id(&wallet.id).await.unwrap();
    assert_eq!(wallet.name, name);

    let wallet = vault.get_wallet_by_name(&wallet.name).await.unwrap();
    assert_eq!(wallet.name, name);
}

#[tokio::test]

async fn can_list_all_wallets() {
    let vault = SqliteVault::new(Some("sqlite::memory:")).await;
    vault.migrate().await.unwrap();
    struct Input {
        name: String,
        password: String,
    }
    let inputs: Vec<Input> = vec![
        Input {
            name: "main".to_string(),
            password: "mainpass".to_string(),
        },
        Input {
            name: "second".to_string(),
            password: "abcd".to_string(),
        },
    ];

    for input in inputs.iter() {
        let mut wallet = WalletInputBuilder::new();
        wallet.name(&input.name);
        wallet.password(&input.password);
        let wallet = wallet.build().unwrap();
        vault.insert_wallet(wallet).await.unwrap();
    }

    let wallets = vault.get_all_wallets().await.unwrap();

    assert_eq!(wallets.len(), inputs.len());
}

#[tokio::test]
async fn can_list_all_accounts_for_wallet() {
    let vault = SqliteVault::new(Some("sqlite::memory:")).await;
    vault.migrate().await.unwrap();

    let mut wallet = WalletInputBuilder::new();

    wallet.name("main");
    wallet.password("password");
    let wallet = wallet.build().unwrap();

    let wallet = vault.insert_wallet(wallet).await.unwrap();

    let key = wallet.authenticate("password").unwrap();

    let paths = [PathBuilder::new().index(0), PathBuilder::new().index(1)];

    for path in paths.iter() {
        let account = AccountInputBuilder::from(wallet.clone())
            .path(path.build())
            .build(key)
            .unwrap();
        vault.insert_account(account.to_owned()).await.unwrap();
    }

    let res = vault.get_all_accounts(&wallet.id).await.unwrap();

    assert_eq!(res.len(), paths.len());
}

#[tokio::test]
async fn can_remove_wallet() {
    let vault = SqliteVault::new(Some("sqlite::memory:")).await;
    vault.migrate().await.unwrap();

    let mut wallet = WalletInputBuilder::new();

    wallet.name("main");
    wallet.password("password");
    let wallet = wallet.build().unwrap();

    let wallet = vault.insert_wallet(wallet).await.unwrap();

    let account = AccountInputBuilder::from(wallet.clone());
    let key = wallet.authenticate("password").unwrap();
    let account = account.build(key).unwrap();
    let account = vault.insert_account(account).await.unwrap();

    let result = vault.remove_wallet_by_id(&wallet.id).await;
    let account = vault.get_account_by_id(&account.id).await;
    assert!(result.is_ok() == true);
    assert!(account.is_err() == true);
}

#[tokio::test]
async fn can_remove_accounts() {
    let vault = SqliteVault::new(Some("sqlite::memory:")).await;
    vault.migrate().await.unwrap();

    let mut wallet = WalletInputBuilder::new();

    wallet.name("main");
    wallet.password("password");
    let wallet = wallet.build().unwrap();

    let wallet = vault.insert_wallet(wallet).await.unwrap();

    let key = wallet.authenticate("password").unwrap();

    let paths = [PathBuilder::new().index(0), PathBuilder::new().index(1)];

    for path in paths.iter() {
        let account = AccountInputBuilder::from(wallet.clone())
            .path(path.build())
            .build(key)
            .unwrap();
        vault.insert_account(account.to_owned()).await.unwrap();
    }

    let res = vault.get_all_accounts(&wallet.id).await.unwrap();

    for account in res.iter() {
        vault.remove_account_by_id(&account.id).await.unwrap();
    }

    let res = vault.get_all_accounts(&wallet.id).await.unwrap();
    assert_eq!(res.len(), 0);
}
