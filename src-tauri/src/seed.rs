use bip39::Mnemonic;
use hex_literal::hex;
use rand::{Rng, RngCore};
use rand_core::{self, OsRng};
use sha2::{Digest, Sha256, Sha512};

pub fn generate_mnemonic() -> Result<Mnemonic, bip39::Error> {
    // 1. Generate entropy (for 24 mnemonic words)
    // 2. Generate checksum (sha256 entrophy and subtract )
    // 3. Add the checksum to the entropy
    // 4. Segmentate the entropy in 24 segments
    // 5. Generate the seed based

    let mut entropy = [0u8; 32];
    let mut rng = OsRng;
    let a: u32 = 00010001000;
    rng.fill_bytes(&mut entropy);
    Mnemonic::from_entropy(&entropy)
}
