use crate::aes_alg;
use crate::helper;

pub fn _encrypt_block(block: &[u8], c:[u8;16], key: &[[u8; 16]; 11] )-> [u8;16]{
    let state = aes_alg::_xor(block.try_into().unwrap(),&c);
    aes_alg::_encrypt_block(&state, *key)
}
pub fn _decrypt_block(block: &[u8], c:[u8;16], key: &[[u8; 16]; 11] )-> [u8;16]{
    let state = aes_alg::_decrypt_block(block, *key);
    aes_alg::_xor(&state,&c)
}

pub fn _encrypt(text : Vec<u8>, key : [u8;16], c:[u8;16]) -> Vec<u8>{
    let _expanded_key = aes_alg::_generate_key(key);
    let mut iv = c;
    let mut res : Vec<u8> = Vec::new();
    for (_index, chunk) in text.chunks(16).enumerate(){
        let state = _encrypt_block(chunk,iv, &_expanded_key);
        res.extend(state);
        iv = state;
    }
    res
}
pub fn _decrypt(text : Vec<u8>, key : [u8;16], c:[u8;16]) -> Vec<u8>{
    let _expanded_key = aes_alg::_generate_key(key);
    let mut iv = c;
    let mut res : Vec<u8> = Vec::new();
    for (_index, chunk) in text.chunks(16).enumerate(){
        let state = _decrypt_block(chunk,iv, &_expanded_key);
        res.extend(state);
        iv = chunk.try_into().unwrap();
    }
    res
}

pub fn _cbc_encrypt(text: String, c:[u8;16], key: String )-> String{
    let mut content = text.as_bytes().to_vec();
    content = aes_alg::_pkcs7_padding(content, 16);
    let _b_key: [u8;16] = key.as_bytes().try_into().unwrap();
    let res = _encrypt(content, _b_key, c);
    helper::_u8_to_hex(res)   
}
pub fn _cbc_decrypt(text: String, c:[u8;16], key: String )-> String{
    let content = helper::_hex_to_u8(text);
    let _b_key: [u8;16] = key.as_bytes().try_into().unwrap();
    let mut res = _decrypt(content, _b_key, c);
    res = aes_alg::_pksc7_unpadding(res);
    String::from_utf8(res).unwrap()
}
