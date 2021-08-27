use std::io::{Read, Write};

use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

// create an alias for convenience
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn encrypt(content: &[u8], key:&[u8], iv:&[u8]) -> Vec<u8>{
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();

    // buffer must have enough space for message+padding
    // copy message to the buffer
    let result = cipher.encrypt_vec(content);
    result
}
pub fn decrypt(encry_content: &[u8], key:&[u8], iv:&[u8]) -> Vec<u8>{
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(encry_content)
            .unwrap();
    decrypted_ciphertext
}
