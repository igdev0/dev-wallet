use bip39::Mnemonic;
use rand::RngCore;
use rand_core::{self, OsRng};
pub fn generate_mnemonic() -> Result<Mnemonic, bip39::Error> {
    // 1. Generate entropy (for 24 mnemonic words)
    // 2. Generate checksum (sha256 entrophy and subtract )
    // 3. Add the checksum to the entropy
    // 4. Segmentate the entropy in 24 segments
    // 5. Generate the seed based

    let mut entropy = [0u8; 32];
    let mut rng = OsRng;
    rng.fill_bytes(&mut entropy);
    Mnemonic::from_entropy(&entropy)
}

// This utility will be used to encrypt fields such as xprv
pub fn encrypt() {}
// This utility will be used to decrypt fields encrypted
pub fn decrypt() {}
