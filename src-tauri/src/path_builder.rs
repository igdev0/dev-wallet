use std::str::FromStr;

use bitcoin::bip32::{ChildNumber, DerivationPath};
use bitcoin::NetworkKind;

pub enum AddressType {
    Receiving,
    Spending,
}

pub enum PathAddressKind {
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
                NetworkKind::Main => ChildNumber::from_hardened_idx(0).unwrap().to_string(),
                NetworkKind::Test => ChildNumber::from_hardened_idx(1).unwrap().to_string(),
            },
        }
    }

    fn purpose(&self) -> String {
        match self.address_kind {
            PathAddressKind::Legacy => ChildNumber::from_hardened_idx(44).unwrap().to_string(),
            PathAddressKind::SegWit => ChildNumber::from_hardened_idx(49).unwrap().to_string(),
            PathAddressKind::NativeSegWit => {
                ChildNumber::from_hardened_idx(84).unwrap().to_string()
            }
        }
    }

    pub fn build(&self) -> DerivationPath {
        // Add purpose
        let purpose = self.purpose();
        let coin_type = self.coin_type();
        let account_index = ChildNumber::from_hardened_idx(self.account_index).unwrap();
        let change_index = ChildNumber::from_normal_idx(self.change_index).unwrap();
        let index = ChildNumber::from_normal_idx(self.index).unwrap();

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
