use std::str::FromStr;

use bip32::{secp256k1::ecdsa::SigningKey, ExtendedPrivateKey};
use bip39::Mnemonic;

pub struct Wallet {
    mnemonic: String,
    passphrase: String,
    seed: [u8; 64],
}

impl Wallet {
    pub fn create_account_key(&self, path: &str) -> ExtendedPrivateKey<SigningKey> {
        let child_xprv = bip32::XPrv::derive_from_path(self.seed, &path.parse().unwrap()).unwrap();
        child_xprv
    }

    pub fn save(&self) {}
}

pub struct WalletBuilder {
    mnemonic: Option<String>,
    xprv: Option<String>,
    passphrase: Option<String>,
}

impl WalletBuilder {
    pub fn new(mnemonic: &str) -> WalletBuilder {
        WalletBuilder {
            mnemonic: Some(mnemonic.to_string()),
            passphrase: Some("".to_string()),
            xprv: Some("".to_string()),
        }
    }

    pub fn passphrase(&mut self, pass: String) {
        self.passphrase = Some(pass);
    }

    pub fn mnemonic(&mut self, mnemonic: String) {
        self.mnemonic = Some(mnemonic);
    }

    pub fn build(mut self) -> Wallet {
        let passphrase = &self.passphrase.unwrap();
        let mnemonic = &self.mnemonic.unwrap();
        let seed = Mnemonic::from_str(&mnemonic)
            .unwrap()
            .to_seed(passphrase.to_string());
        Wallet {
            mnemonic: mnemonic.to_string(),
            passphrase: passphrase.to_string(),
            seed,
        }
    }
}
