use bip39::Mnemonic;
use bitcoin::hex::{Case, DisplayHex};
use rand::RngCore;
use rand_core::{self, OsRng};

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use thiserror::Error;

use crate::utils::{encrypt, AESError, AESKey};

#[derive(Debug)]
pub struct WalletModel {
    pub id: String,
    pub name: String,
    pub password: String,
    pub seed: String,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Authentication failed")]
    Failed,
    #[error("Parser failed {0}")]
    Parser(String),
}

pub type AuthResult = Result<AESKey, AuthError>;

impl WalletModel {
    pub fn authenticate(&self, password: &str) -> AuthResult {
        let argon2 = Argon2::default();
        let parsed_password = PasswordHash::new(&self.password);

        if let Err(err) = parsed_password {
            return Err(AuthError::Parser(err.to_string()));
        }

        let parsed_password = parsed_password.unwrap();

        let auth_result = argon2.verify_password(password.as_bytes(), &parsed_password);
        if let Err(_) = auth_result {
            return Err(AuthError::Failed);
        }

        let mut key = [0u8; 32];
        let hash = parsed_password.hash.unwrap();

        key.copy_from_slice(&hash.as_bytes()[..32]);
        Ok(key)
    }
}

impl From<StoreWalletInput> for WalletModel {
    fn from(value: StoreWalletInput) -> WalletModel {
        WalletModel {
            id: uuid::Uuid::new_v4().to_string(),
            name: value.name,
            password: value.encrypted_pass,
            seed: value.encrypted_seed,
        }
    }
}

#[derive(Default, Debug)]
pub struct StoreWalletInput {
    pub name: String,
    pub encrypted_pass: String,
    pub encrypted_seed: String,
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
    pub fn new() -> Self {
        let mut entropy = [0u8; 32];
        let mut rng = OsRng;
        rng.fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy(&entropy).expect("Mnemonic generation fail");
        Self {
            mnemonic,
            name: "".to_string(),
            password: "".to_string(),
        }
    }

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

    pub fn mnemonic_as_string(&self) -> String {
        self.mnemonic.to_string()
    }

    pub fn build(&self) -> Result<StoreWalletInput, AESError> {
        let salt = SaltString::generate(&mut OsRng);
        let hasher = Argon2::default();
        let password = hasher
            .hash_password(&self.password.as_bytes(), &salt)
            .expect("Failed hashing the password");

        let aes_key = &password.clone().hash.unwrap();
        let mut key: [u8; 32] = [0u8; 32];
        key.copy_from_slice(&aes_key.as_bytes()[..32]);

        let seed = self.mnemonic.to_seed(&self.password);

        let encrypted_seed = encrypt(&key, &seed)?;
        Ok(StoreWalletInput {
            encrypted_pass: password.to_string(),
            encrypted_seed: encrypted_seed.to_hex_string(Case::Lower),
            name: self.name.to_string(),
        })
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

        let wallet_input = res.build().unwrap();

        assert_eq!(wallet_input.name, "name");
        assert_ne!(wallet_input.encrypted_pass, "password"); // is encrypted now

        let model = WalletModel::from(wallet_input);

        assert!(model.id.len() > 0);

        assert!(model.seed.len() > 0);
    }

    #[test]
    fn can_create_wallet_input_from_mnemonic() {
        let mut entropy = [0u8; 32];
        let mut rng = OsRng;
        rng.fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy(&entropy).expect("Mnemonic generation fail");

        let res = WalletInputBuilder::from(mnemonic);
        assert!(res.mnemonic.to_string().len() > 0);

        let wallet_input = res.build().unwrap();

        let model = WalletModel::from(wallet_input);

        assert!(model.id.len() > 0);

        assert!(model.seed.len() > 0);
    }

    #[test]
    fn can_create_wallet_input_from_new() {
        let res = WalletInputBuilder::new();
        assert!(res.mnemonic.to_string().len() > 0);

        let wallet_input = res.build().unwrap();

        let model = WalletModel::from(wallet_input);

        assert!(model.id.len() > 0);

        assert!(model.seed.len() > 0);
    }
}
