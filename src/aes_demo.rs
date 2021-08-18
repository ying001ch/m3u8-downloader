use std::io::{Read, Write};

use aes::Aes128;
use aes::cipher::generic_array::{ArrayLength, GenericArray};
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};

// create an alias for convenience
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn get_file_cnt(path:&str) -> Vec<u8> {
    // let path = "C:\\Users\\frank\\Desktop\\v.f56150.ts";
    let mut file = std::fs::File::open(path).expect("file open failed");
    let mut v:Vec<u8> = Vec::new();
    let size = file.read_to_end(&mut v).expect("file read failed");
    println!("size={}", size);
    v
}
fn write_file(content:&[u8]) {
    let path = "C:\\Users\\frank\\Desktop\\_v.f56150.ts";
    let mut file = std::fs::File::create(path).expect("open file failed");
    let handler = file.write(content);
    handler.expect("写入失败");
}

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
