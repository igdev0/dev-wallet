use std::str::FromStr;

use bip32::{ChildNumber, DerivationPath};
use bitcoin::{Network, NetworkKind};

use crate::WalletError;

enum AddressType {
    Receiving,
    Spending,
}

enum PathAddressKind {
    Legacy,       // P2PKH
    SegWit,       // P2SH-P2WPKH
    NativeSegWit, // P2WPKH
}

pub enum SupportedNetworks {
    Bitcoin,
}

pub struct PathBuilder {
    pub network: SupportedNetworks,
    pub address_type: AddressType,
    pub address_kind: PathAddressKind,
    pub account_index: u32,
    pub change_index: u32,
    pub index: u32,
    pub network_kind: NetworkKind,
}

// m / purpose' / coin_type' / account' / change / address_index

impl PathBuilder {
    pub fn new() -> PathBuilder {
        PathBuilder {
            address_type: AddressType::Receiving,
            address_kind: PathAddressKind::SegWit,
            network: SupportedNetworks::Bitcoin,
            network_kind: NetworkKind::Main,
            account_index: 0,
            change_index: 0,
            index: 0,
        }
    }

    pub fn network(mut self, network: SupportedNetworks) {
        self.network = network;
    }

    pub fn network_kind(mut self, kind: NetworkKind) {
        self.network_kind = kind;
    }

    pub fn index(mut self, index: u32) {
        self.index = index;
    }

    pub fn account_index(mut self, index: u32) {
        self.account_index = index;
    }

    pub fn change_index(mut self, index: u32) {
        self.change_index = index;
    }

    pub fn coin_type(&self) -> String {
        match &self.network {
            SupportedNetworks::Bitcoin => match &self.network_kind {
                NetworkKind::Main => ChildNumber::new(0, true).unwrap().to_string(),
                NetworkKind::Test => ChildNumber::new(1, true).unwrap().to_string(),
            },
        }
    }

    fn purpose(&self) -> String {
        match self.address_kind {
            PathAddressKind::Legacy => ChildNumber::new(44, true).unwrap().to_string(),
            PathAddressKind::SegWit => ChildNumber::new(49, true).unwrap().to_string(),
            PathAddressKind::NativeSegWit => ChildNumber::new(84, true).unwrap().to_string(),
        }
    }

    pub fn build(&self) -> DerivationPath {
        // Add purpose
        let purpose = self.purpose();
        let coin_type = self.coin_type();
        let account_index = ChildNumber::new(self.account_index, true).unwrap();
        let change_index = ChildNumber::new(self.change_index, true).unwrap();
        let index = ChildNumber::new(self.index, true).unwrap();

        DerivationPath::from_str(
            format!(
                "m/{}/{}/{}/{}/{}",
                purpose, coin_type, account_index, change_index, index
            )
            .as_str(),
        )
        .expect("Wasn't able to create a derivation path")
    }
}
