use std::collections::HashSet;
use std::sync::OnceLock;

use rand::RngExt;

use crate::cbc_alg;
use crate::conversion;
use crate::aes_alg;
use crate::helper;




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
pub fn _cbc_file_decryption(file: &str, key: String) -> Result<String, String> {
    let key_bytes = key.as_bytes();

    if key_bytes.len() != 16 {
        return Err("Key must be 16 bytes".to_string());
    }

    let mut _bkey = [0u8; 16];
    _bkey.copy_from_slice(key_bytes);

    let content64 = aes_alg::_convert_file(file);
    let content = helper::_u8_to_hex(content64);

    Ok(cbc_alg::_cbc_string_decrypt(content, [0u8;16], _bkey))
}


//-----------------------------------------------------------Challenge 11



fn _aes_key()-> [u8;16]{
    let mut key  = [0u8;16];
    rand::fill(&mut key);
    key
}
fn _encryption_oracle(text : Vec<u8>) -> String{
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
    let mut data : Vec<u8> = Vec::new();
    data.extend(pre_slice);
    data.extend(text);
    data.extend(post_slice);
    
    
    let use_ecb = rand::random::<bool>();
    let key = _aes_key();
    let mut _res =String::new();
    if use_ecb{ //ECB
        _res = aes_alg::_aes_encrypt(data, key);
    }
    else { //CBC
        let mut iv = [0u8;16];
        rand::fill(&mut iv);
        _res = cbc_alg::_cbc_encrypt(data, iv, key);
    }
    println!("{}",_res);
    _res
}

pub fn _detect_oracle(text: Vec<u8>) -> String {
    let ciphertext = _encryption_oracle(text);
    let mut set: HashSet<[u8; 16]> = HashSet::new();
    let res = ciphertext.as_bytes().to_vec();
    for chunk in res.chunks(16) {
        let block: [u8; 16] = chunk.try_into().unwrap();
        if !set.insert(block) {
            return "ECB".to_string();
        }
    }
    "CBC".to_string()
}

//-----------------------------------------------------------Challenge 12

static _GLOBAL_KEY: OnceLock<[u8;16]> = OnceLock::new();
fn _get_key() -> &'static [u8;16] {
    _GLOBAL_KEY.get_or_init(|| _aes_key())
}

fn _buffer_encrypt(mut text: Vec<u8>)-> Vec<u8>{
    let s = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";

    let suffix = helper::_base64_to_u8(s.to_string());
    text.extend(suffix);
    aes_alg::_encrypt(text, *_get_key())
}
 
fn _figure_out_first_byte() -> u8{
    let block = vec![b'A';15];
    let mut first_byte = 0 as u8;
    let res_block = _buffer_encrypt(block.clone());
    for i in 0u8..=0xff{
        let mut test_block = block.clone();
        test_block.push(i);
        if _buffer_encrypt(test_block)[..16] == res_block[..16]{
            first_byte = i;
            break;
        }
    }
    first_byte
}
fn _figure_out_first_block() -> Vec<u8> {
    let mut known: Vec<u8> = Vec::new();
    for i in 0..16 {
        let padding = vec![b'A'; 15 - i];
        let target = _buffer_encrypt(padding.clone())[..16].to_vec();
        
        let mut found = false;
        for j in 0u8..=0xff {
            let mut test_block = padding.clone();
            test_block.extend(&known);
            test_block.push(j);
            if _buffer_encrypt(test_block)[..16] == target {
                known.push(j);
                found = true;
                break;
            }
        }
        if !found { break; }
    }
    known
}

fn _decrypt_oracle() -> Vec<u8> {
    let num_blocks = _buffer_encrypt(vec![]).len() / 16;
    let mut known: Vec<u8> = Vec::new();
    
    for index in 0..num_blocks {
        for i in 0..16 {
            let padding = vec![b'A'; 15 - i];
            let target = _buffer_encrypt(padding.clone())[(16*index)..(16*(index+1))].to_vec();
            
            let mut prefix: Vec<u8> = Vec::new();
            prefix.extend(&padding);
            prefix.extend(&known);
            let prefix = prefix[prefix.len() - 15..].to_vec();
            
            let mut found = false;
            for j in 0u8..=0xff {
                let mut test_block = prefix.clone();
                test_block.push(j);
                if _buffer_encrypt(test_block)[..16] == target {
                    known.push(j);
                    found = true;
                    break;
                }
            }
            if !found { break; }
        }
    }
    known
}







#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _cipher_length(){
        let mut c: Vec<u8> = Vec::new();
        for i in 1..=32{
            c.push(b'A');
            println!("Input len: {} => Cipher length: {}", i, _buffer_encrypt(c.clone()).len());
        }
    }
    
    fn _ecb_or_no() -> bool{
        let v: Vec<u8> = vec![b'A'; 32];  
        let cipher = _buffer_encrypt(v.clone());
        let mut set: HashSet<[u8; 16]> = HashSet::new();
        for chunk in cipher.chunks(16) {
            let block: [u8; 16] = chunk.try_into().unwrap();
            if !set.insert(block) {
                return true;
            }
        }
        false
    }
    #[test]
    fn test_ecb_detection() {
        assert!(_ecb_or_no());
    }
    #[test]
    fn incomplete_block_encryption(){
        println!("{}",_figure_out_first_byte())
    }
    #[test]
    fn block_encryption(){
        println!("{}", String::from_utf8(_figure_out_first_block()).unwrap());
    }
    #[test]
    fn text_encryption(){
        println!("{}", String::from_utf8(_decrypt_oracle()).unwrap());
    }
}




























































