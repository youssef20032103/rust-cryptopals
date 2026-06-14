
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

fn _buffer_encrypt(text: Vec<u8>)-> Vec<u8>{
    let s = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let suffix = helper::_base64_to_u8(s.to_string());
    let mut data = text.clone();
    data.extend(suffix);
    let key = *_get_key();
    aes_alg::_encrypt(data, key)
}

fn _decrypt_next_byte(recovered: &[u8]) -> u8 {        //prefix + 38 As = 48 char
    let block_num = recovered.len() / 16;       // block_num = 3
    let index = recovered.len() % 16;           // index = 0
    let pad_len = 15 - index;                   // pad = 15 As

    let pad = vec![b'A'; pad_len];            // 15 As

    let target = _buffer_encrypt(pad.clone()); 

    for guess in 0u8..=255 {
        let mut test = pad.clone();

        test.extend_from_slice(recovered);
        test.push(guess);

        let encrypted = _buffer_encrypt(test);

        if encrypted[16 * block_num..16 * (block_num + 1)]
            == target[16 * block_num..16 * (block_num + 1)]
        {
            return guess;
        }
    }

    panic!("No matching byte found");
}

fn _decrypt() -> Vec<u8> {
    
    let mut recovered = Vec::new();
    for _ in 0..139 {
        let byte = _decrypt_next_byte(&recovered);
        recovered.push(byte);
    }

    recovered
}


//-----------------------------------------------------------Challenge 13

struct Profile {
    email: String,
    uid: String,
    role: String,
}

fn _get_key_value(parsed: String) -> Profile{
    let res : Vec<&str> = parsed.split('&').collect();
    let mut items : Vec<String> = Vec::new();
    for i in res {
        let pair :Vec<&str>= i.split('=').collect();
        items.push(pair[1].to_string());
    }
    Profile{email: items[0].clone(), uid: items[1].clone(), role: items[2].clone()}
}

fn _map_encode(profile : Profile) -> String{
    format!("email={}&uid={}&role={}", profile.email, profile.uid, profile.role)
}

fn _profile_for(mut email:String) -> String{
    email = email.replace('=', "").replace('&', "");
    let p = Profile{email: email.clone(),
                             uid:"10".to_string(),
                             role: "user".to_string()};
    
    _map_encode(p)
}

fn _encrypt_profile(profile: String) -> (Vec<u8>, [u8;16]){
    let cipher = profile.as_bytes().to_vec();
    let key = _aes_key();
    (aes_alg::_encrypt(cipher, key), key)
}
fn _decrpypt_profile(cipher: Vec<u8>, key: [u8;16])-> String{
    let p = aes_alg::_aes_decrypt(cipher, key);
    let res = _get_key_value(p);
    _map_encode(res)
}

fn _attack_cipher()-> String{
    let entry = "AAAAAAAAAAAAA".to_string();
    let blocks = (_profile_for(entry)).as_bytes().to_vec();
    let key = _aes_key();
    let res = ((aes_alg::_encrypt(blocks, key))[..32]).to_vec();
    let entry2 = "AAAAAAAAAAadmin\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b".to_string();
    let blocks2 = (_profile_for(entry2)).as_bytes().to_vec();
    let res2 = ((aes_alg::_encrypt(blocks2, key))[16..32]).to_vec();
    let mut cipher = res.clone();
    cipher.extend(res2);
    _decrpypt_profile(cipher,key)
}

//-----------------------------------------------------------Challenge 14
static _PREFIX: OnceLock<[u8;17]> = OnceLock::new();
fn _generate_prefix()-> [u8;17]{
    let mut key  = [0u8;17];
    rand::fill(&mut key);
    key
}
fn _get_prefix() -> &'static [u8;17] {
    _PREFIX.get_or_init(|| _generate_prefix())
}


fn _encrypt_oracle_14(text: Vec<u8>) -> Vec<u8>{
    let pad = _get_prefix().to_vec();
    let padded_text : Vec<u8> = [pad,text].concat();
    _buffer_encrypt(padded_text)
}

fn _matrice_number(text: Vec<u8>) -> (bool, usize){
    let encrypted = _encrypt_oracle_14(text.clone());
    let chunks: Vec<&[u8]> = encrypted.chunks(16).collect();
    for index in 0..chunks.len().saturating_sub(1){
        if chunks[index] == chunks[index+1]{
            return (true, index);
        }
    }
    (false, 0)
}

fn _decrypt_next_byte_14(recovered: &[u8], offset:usize, offset_block:usize) -> u8 {        
    let block_num = recovered.len()/16 + offset_block;       
    let index = recovered.len() % 16;           
    let pad_len = 15 - index;                   

    let pad = vec![b'A'; pad_len + offset];            

    let target = _encrypt_oracle_14(pad.clone()); 

    for guess in 0u8..=255 {
        let mut test = pad.clone();

        test.extend_from_slice(recovered);
        test.push(guess);

        let encrypted = _encrypt_oracle_14(test);

        if encrypted[16 * block_num..16 * (block_num + 1)]
            == target[16 * block_num..16 * (block_num + 1)]
        {
            return guess;
        }
    }

    panic!("No matching byte found");
}




fn _decrypt_14()-> Vec<u8>{
    let mut offset  = 0;
    let mut offset_block = 0;
    for n in 0..32 {
        let input = vec![b'A'; n + 32];
        if _matrice_number(input.clone()).0 {
            offset_block = _matrice_number(input.clone()).1;
            offset = n  ;
            break;
        }
    }
    let mut recovered = Vec::new();
    for _ in 0..139 {
        let byte = _decrypt_next_byte_14(&recovered, offset, offset_block);
        recovered.push(byte);
    }

    recovered
    
}


//-----------------------------------------------------------Challenge 15
fn _pkcs7_unpadding(padded: Vec<u8>) -> Result<Vec<u8>, String>{
    let k = padded[padded.len()-1] as usize;
    if k == 0 || k > 16 || k > padded.len() {
        return Err("invalid padding".into());
    }
    for i in 0..k {
        if padded[padded.len()-1-i] as usize != k {
            return Err("invalid padding".into());
        }
    }
    Ok(padded[..padded.len()-k].to_vec())
}
//-----------------------------------------------------------Challenge 16

static _IV: OnceLock<[u8;16]> = OnceLock::new();
fn _iv()-> [u8;16]{
    let mut iv  = [0u8;16];
    rand::fill(&mut iv);
    iv
}
fn _get_iv() -> &'static [u8;16] {
    _IV.get_or_init(|| _iv())
}


fn _cbc_flip_encrypt(text: Vec<u8>) -> Vec<u8>{
    let mut data = "comment1=cooking%20MCs;userdata=".as_bytes().to_vec();
    data.extend(text.into_iter()
                    .filter(|&b| b != b';' && b != b'=')
                    .collect::<Vec<u8>>());
    data.extend(";comment2=%20like%20a%20pound%20of%20bacon".as_bytes().to_vec());
    

    cbc_alg::_encrypt(data, *_get_key(), *_get_iv())
}















#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _cipher_length(){
        let mut c: Vec<u8> = Vec::new();
        for i in 1..=32{
            c.push(b'A');
            let res =_buffer_encrypt(c.clone());
            println!("Input len: {} => Cipher length: {}", i, res.len());
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
    fn text_encryption(){
        let res = _decrypt();
        println!("{}", String::from_utf8_lossy(&res));
    }
    #[test]
    fn parse(){
        println!("{}", _profile_for("yuna@gmail.com".to_string()));
    }
    #[test]
    fn encrypt_decrypt(){
        let pr = _profile_for("Yuna@gmail.com".to_string());
        let encrypted = _encrypt_profile(pr);
        let key = encrypted.1;
        println!("(key = {:?},profile = {})", key, _decrpypt_profile(encrypted.0,encrypted.1) );
    }
    #[test]
    fn attack(){
        println!("{}", _attack_cipher());
    }
    #[test]
    fn decrypt_oracle_14(){
        let res = _decrypt_14();
        println!("{}", String::from_utf8_lossy(&res));
    }
   
    
}
























































