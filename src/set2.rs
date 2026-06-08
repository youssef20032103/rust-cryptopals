use rand::RngExt;

use crate::cbc_alg;
use crate::conversion;
use crate::aes_alg;
use crate::helper;
use std::fs;



//-----------------------------------------------------------Challenge 9
pub fn _padding(text : String){
    let text = text
    .replace('\n', "")
    .replace('\r', "")
    .replace(' ', "");
    let mut _text = conversion::_base64_decode(text);
    let _rest =16-(_text.len()%16);
    
    
    for _ in 0.._rest{
        _text.push(_rest as u8);
    }
    
}

//-----------------------------------------------------------Challenge 10
pub fn _cbc_file_decryption(file: &str, key: String)-> String{
    let content64 = aes_alg::_convert_file(file);
    let content = helper::_u8_to_hex(content64);
    cbc_alg::_cbc_decrypt(content, [0u8;16], key)
}


//-----------------------------------------------------------Challenge 11
struct Key{
    string_litteral : String,
    byte_list : Vec<u8>
}


fn _aes_key()-> Key{
    let mut key  = [0u8;16];
    rand::fill(&mut key);
    (String::from_utf8(key), key)
}
fn _encryption_oracle(text : String){
    let content = text.into_bytes();
    let mut r = rand::rng();
    let pre = r.random_range(5..=10);
    let post = r.random_range(5..=10);
    let mut pre_slice : Vec<u8> = Vec::new();
    let mut post_slice : Vec<u8> = Vec::new();
    for _ in 0..pre{
        pre_slice.push(r.random::<u8>());
    }
    for _ in 0..post{
        post_slice.push(r.random::<u8>());
    }
    pre_slice.extend(content);
    pre_slice.extend(post_slice);
    let use_ecb = rand::random::<bool>();
    let key = _aes_key();
    if use_ecb{
        aes_alg::_aes_encrypt(text, key)
    }
    else { //EBC
        }
    
}
