// use aes::Aes128;
// use block_modes::block_padding::Pkcs7;
// use block_modes::{BlockMode, Cbc};

// // create an alias for convenience
// type Aes128Cbc = Cbc<Aes128, Pkcs7>;

use anyhow::Ok;

fn encrypt(content: &[u8], key:&[u8], iv:&[u8]) -> Vec<u8>{
//     let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();

//     // buffer must have enough space for message+padding
//     // copy message to the buffer
//     let result = cipher.encrypt_vec(content);
//     result
    vec![]
}
pub fn decrypt(encry_content: &[u8], key:&[u8], iv:&[u8]) -> Result<Vec<u8>, String>{
//     let cipher = Aes128Cbc::new_from_slices(key, iv).unwrap();
//     match cipher.decrypt_vec(encry_content){
//         Ok(t)=>Ok(t),
//         Err(e)=>Err(e.to_string())
//     }
    Err("".to_string())
}
#[test]
fn test_decrypt(){
    use aes::Aes128;
    use aes::cipher::{
        BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
        generic_array::GenericArray,
    };

    let key = GenericArray::from([0u8; 16]);
    let mut block = GenericArray::from([42u8; 16]);

    // Initialize cipher
    let cipher = Aes128::new(&key);

    let block_copy = block.clone();

    // // Encrypt block in-place
    // cipher.encrypt_block(&mut block);

    // // And decrypt it back
    // cipher.decrypt_block(&mut block);
    // assert_eq!(block, block_copy);
    
    //多块解密
    let blcs = [block;19];
    let mut blcs_out_en = [block;19];
    let mut blcs_out_de = [key;19];
    cipher.encrypt_blocks_b2b(& blcs,&mut blcs_out_en).unwrap();
    cipher.decrypt_blocks_b2b(& blcs_out_en, &mut blcs_out_de).unwrap();

    println!("blcs_out_en[0]{:?}",blcs_out_en[0]);
    println!("blcs_out_de[0]{:?}",blcs_out_de[0]);
    
    for bl in blcs_out_de{
        assert_eq!(bl, block_copy);
    }
    let mut temp = [0u8;16];
    temp.copy_from_slice(&blcs_out_de[0][..]);
    println!("temp {:?}",temp);
    let mut res = vec![0u8;2 * 16];

    res[..16].copy_from_slice(&blcs_out_de[0][..]);
    (&mut res[16..32]).copy_from_slice(&blcs_out_de[0][..]);
    println!("total {:?}", res);
}