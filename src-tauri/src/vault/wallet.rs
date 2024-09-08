use bip39::Mnemonic;
use bitcoin::hex::{Case, DisplayHex};
use rand::RngCore;
use rand_core::{self, OsRng};

use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

use crate::utils::encrypt;

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

    pub fn build(&self) -> StoreWalletInput {
        let salt = SaltString::generate(&mut OsRng);
        let hasher = Argon2::default();
        let password = hasher
            .hash_password(&self.password.as_bytes(), &salt)
            .expect("Failed hashing the password");

        let aes_key = &password.clone().hash.unwrap();
        let mut key: [u8; 32] = [0u8; 32];
        key.copy_from_slice(&aes_key.as_bytes()[..32]);

        let seed = self.mnemonic.to_seed(&self.password);

        StoreWalletInput {
            encrypted_pass: password.to_string(),
            encrypted_seed: encrypt(&key, &seed).to_hex_string(Case::Lower),
            name: self.name.to_string(),
        }
    }
}

impl StoreWalletInput {
    pub fn new<'a>(name: &'a str, password: &'a str) -> WalletInputBuilder {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_create_wallet_input_from_name_and_password() {
        let res = StoreWalletInput::new("name", "password");
        assert_eq!(res.name, "name");
        assert_eq!(res.password, "password"); // not encrypted
        assert!(res.mnemonic.to_string().len() > 0);

        let wallet_input = res.build();

        assert_eq!(wallet_input.name, "name");
        assert_ne!(wallet_input.encrypted_pass, "password"); // is encrypted now

        let model = WalletModel::from(wallet_input);

        assert!(model.id.len() > 0);

        assert!(model.seed.len() > 0);
    }
}
