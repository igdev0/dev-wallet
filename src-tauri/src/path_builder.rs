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

    pub fn network_kind(mut self, kind: NetworkKind) -> Self {
        self.network_kind = kind;
        self
    }

    pub fn index(mut self, index: u32) -> Self {
        self.index = index;
        self
    }

    pub fn account_index(mut self, index: u32) -> Self {
        self.account_index = index;
        self
    }

    pub fn change_index(mut self, index: u32) -> Self {
        self.change_index = index;
        self
    }

    pub fn coin_type(&self) -> ChildNumber {
        match &self.network {
            SupportedNetworks::Bitcoin => match &self.network_kind {
                NetworkKind::Main => ChildNumber::from_hardened_idx(0).unwrap(),
                NetworkKind::Test => ChildNumber::from_hardened_idx(1).unwrap(),
            },
        }
    }

    fn purpose(&self) -> ChildNumber {
        match self.address_kind {
            PathAddressKind::Legacy => ChildNumber::from_hardened_idx(44).unwrap(),
            PathAddressKind::SegWit => ChildNumber::from_hardened_idx(49).unwrap(),
            PathAddressKind::NativeSegWit => ChildNumber::from_hardened_idx(84).unwrap(),
        }
    }

    pub fn build(&self) -> DerivationPath {
        let purpose = self.purpose();
        let coin_type = self.coin_type();
        let account_index = ChildNumber::from_hardened_idx(self.account_index).unwrap();
        let change_index = ChildNumber::from_normal_idx(self.change_index).unwrap();
        let index = ChildNumber::from_normal_idx(self.index).unwrap();

        DerivationPath::from(vec![purpose, coin_type, account_index, change_index, index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_build_bip32_path() {
        let path = PathBuilder::new();
        let path = path.build().to_string();

        assert_eq!(path, "49'/0'/0'/0/0");
    }
}
