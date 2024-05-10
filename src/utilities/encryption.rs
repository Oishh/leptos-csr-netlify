pub fn enc(plain_text: String) -> String {
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    let encryption_key = option_env!("JABRAKEY");
    let magic_crypt = new_magic_crypt!(encryption_key.unwrap_or_default(), 256);
    magic_crypt.encrypt_str_to_base64(plain_text)
}

pub fn dec(encrypted_text: String) -> String {
    use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    let encryption_key = option_env!("JABRAKEY");
    let magic_crypt = new_magic_crypt!(encryption_key.unwrap_or_default(), 256);
    magic_crypt
        .decrypt_base64_to_string(encrypted_text)
        .unwrap()
}
