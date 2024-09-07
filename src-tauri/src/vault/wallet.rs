use bip39::Mnemonic;
use rand::RngCore;
use rand_core::{self, OsRng};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use bip39::Mnemonic;
use rand_core::OsRng;

#[derive(Default, Debug)]
pub struct WalletModel {
    pub id: String,
    pub name: String,
    pub password: String,
    pub seed: String,
}

impl From<StoreWalletInput> for WalletModel {
    fn from(value: StoreWalletInput) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: value.name,
            password: value.encrypted_pass,
            seed: value.encrypted_seed,
        }
    }
}

#[derive(Default, Debug)]
pub struct StoreWalletInput {
    name: String,
    encrypted_pass: String,
    encrypted_seed: String,
}

#[derive(Debug)]
pub struct WalletInputBuilder {
    name: String,
    password: String,
    mnemonic: Mnemonic,
}

impl From<Mnemonic> for WalletInputBuilder {
    fn from(value: Mnemonic) -> Self {
        Self {
            name: String::new(),
            password: String::new(),
            mnemonic: value,
        }
    }
}

impl WalletInputBuilder {
    pub fn name(&mut self, name: &str) {
        self.name = name.to_string()
    }

    pub fn password(&mut self, password: &str) {
        self.password = password.to_string()
    }

    pub fn regenerate_mnemonic(&mut self) {
        let mut entropy = [0u8; 32];
        let mut rng = OsRng;
        rng.fill_bytes(&mut entropy);

        self.mnemonic = Mnemonic::from_entropy(&entropy).expect("Mnemonic generation fail");
    }

    pub fn build() -> StoreWalletInput {
        StoreWalletInput {
            ..Default::default()
        }
    }
}

impl StoreWalletInput {
    fn new<'a>(name: &'a str, password: &'a str) -> WalletInputBuilder {
        let mut entropy = [0u8; 32];
        let mut rng = OsRng;
        rng.fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy(&entropy).expect("Mnemonic generation fail");

        WalletInputBuilder {
            name: name.to_string(),
            password: password.to_string(),
            mnemonic,
        }
    }
}
