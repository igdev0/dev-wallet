use bip39::Mnemonic;
use bitcoin::hex::{Case, DisplayHex};
use dev_wallet::{
    path_builder::PathBuilder,
    utils::{decrypt, encrypt},
};
use rand::RngCore;
use rand_core::{self, OsRng};

#[test]
fn can_build_bip32_path() {
    let path = PathBuilder::new();
    let path = path.build().to_string();

    assert_eq!(path, "49'/0'/0'/0/0");
}
#[test]
fn can_encrypt_and_decrypt_data() {
    let key = [1u8; 32];

    let text = b"Hello world";

    let encrypted_data = encrypt(&key, text);

    let decrypted = decrypt(&key, &encrypted_data);
    println!("{}", &decrypted.to_hex_string(Case::Lower));
    let decrypted = decrypted.to_hex_string(Case::Lower);
    let text = text.to_hex_string(Case::Lower);
    assert_eq!(text, decrypted);
}
