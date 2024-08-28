use bip39::Mnemonic;
use rand::RngCore;
use rand_core::{self, OsRng};

use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{AeadCore, Aes256Gcm, Nonce}; // Or `Aes128Gcm`

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

/// Encrypts the given plaintext using AES-GCM.
///
/// # Arguments
///
/// * `key` - The 32-byte key for AES-256 encryption.
/// * `plaintext` - The data to be encrypted.
///
/// # Returns
///
/// An vector containing the nonce and the ciphertext as bytes.
pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Vec<u8> {
    // Create AES-GCM cipher
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));

    // Generate a random nonce
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .expect("encryption failure!");

    [nonce.as_slice(), ciphertext.as_slice()].concat()
}

/// Decrypts the given ciphertext using AES-GCM.
///
/// # Arguments
///
/// * `key` - The 32-byte key for AES-256 decryption.
/// * `encrypted_data` - The nonce used during encryption + the data to be decrypted.
///
/// # Returns
///
/// The decrypted plaintext.
pub fn decrypt(key: &[u8; 32], encrypted_data: &[u8]) -> Vec<u8> {
    // Create AES-GCM cipher
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));

    // Convert nonce slice to Nonce type
    let (nonce, ciphertext) = encrypted_data.split_at(12); // 96-bits; unique per message
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .expect("decryption failure!");

    plaintext
}
