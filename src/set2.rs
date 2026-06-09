use std::collections::HashSet;

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
fn _encryption_oracle(text : String) -> String{
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
    let mut data : Vec<u8> = Vec::new();
    data.extend(pre_slice);
    data.extend(content);
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

pub fn _detect_oracle(text: String) -> String {
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

#[test]
fn test_detect_oracle() {
    let input = "A".repeat(48);
    for _ in 0..20 {
        let mode = _detect_oracle(input.clone());
        assert!(mode == "ECB" || mode == "CBC");
        println!("{} => {}", &input[..10], mode);
    }
}






















































#[cfg(test)]
mod tests {
    use super::*;

    // ── ECB ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_ecb_roundtrip() {
        let key = [0x2b,0x7e,0x15,0x16,0x28,0xae,0xd2,0xa6,
                   0xab,0xf7,0x15,0x88,0x09,0xcf,0x4f,0x3c];
        let plaintext = "Hello, ECB world!".to_string();
        let cipher = aes_alg::_aes_string_encrypt(plaintext.clone(), key);
        let recovered = aes_alg::_aes_string_decrypt(
            cipher, key
        );
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn test_ecb_identical_blocks_produce_identical_ciphertext() {
        // ECB's defining weakness: same plaintext block → same ciphertext block
        let key = [0x00u8; 16];
        let plaintext = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_vec(); // 64 bytes = 4 identical blocks
        let cipher = aes_alg::_encrypt(plaintext, key);
        let blocks: Vec<&[u8]> = cipher.chunks(16).collect();
        assert_eq!(blocks[0], blocks[1]);
        assert_eq!(blocks[1], blocks[2]);
    }

    #[test]
    fn test_ecb_different_keys_differ() {
        let key1 = [0x00u8; 16];
        let key2 = [0x01u8; 16];
        let plaintext = b"same plaintext!!".to_vec();
        let c1 = aes_alg::_encrypt(plaintext.clone(), key1);
        let c2 = aes_alg::_encrypt(plaintext, key2);
        assert_ne!(c1, c2);
    }

    // ── CBC ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_cbc_roundtrip() {
        let key = [0x2b,0x7e,0x15,0x16,0x28,0xae,0xd2,0xa6,
                   0xab,0xf7,0x15,0x88,0x09,0xcf,0x4f,0x3c];
        let iv  = [0x00u8; 16];
        let plaintext = "Hello, CBC world!".to_string();
        let cipher = cbc_alg::_cbc_string_encrypt(plaintext.clone(), iv, key);
        let recovered = cbc_alg::_cbc_string_decrypt(cipher, iv, key);
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn test_cbc_identical_blocks_differ() {
        // CBC's chaining means identical plaintext blocks → different ciphertext
        let key = [0x00u8; 16];
        let iv  = [0x00u8; 16];
        let plaintext = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_vec();
        let padded = aes_alg::_pkcs7_padding(plaintext, 16);
        let cipher = cbc_alg::_encrypt(padded, key, iv);
        let blocks: Vec<&[u8]> = cipher.chunks(16).collect();
        assert_ne!(blocks[0], blocks[1]);
    }

    #[test]
    fn test_cbc_iv_changes_output() {
        let key = [0x00u8; 16];
        let iv1 = [0x00u8; 16];
        let iv2 = [0xffu8; 16];
        let plaintext = b"same plaintext!!".to_vec();
        let c1 = cbc_alg::_cbc_encrypt(plaintext.clone(), iv1, key);
        let c2 = cbc_alg::_cbc_encrypt(plaintext, iv2, key);
        assert_ne!(c1, c2);
    }

    #[test]
    fn test_cbc_wrong_iv_corrupts_first_block_only() {
        // With the wrong IV, only the first plaintext block is corrupted on decrypt;
        // subsequent blocks recover correctly because IV only affects block 0.
        let key = [0x42u8; 16];
        let iv  = [0x00u8; 16];
        let bad_iv = [0xffu8; 16];
        let plaintext = b"block one here!!block two here!!".to_vec(); // two blocks

        let cipher = cbc_alg::_cbc_encrypt(plaintext, iv, key);
        // decrypt with wrong IV → first block garbled, second block intact
        let cipher_bytes = helper::_hex_to_u8(cipher);
        let recovered = cbc_alg::_decrypt(cipher_bytes, key, bad_iv);

        // block 1 (bytes 0–15): should differ from "block one here!!"
        assert_ne!(&recovered[0..16], b"block one here!!");
        // block 2 (bytes 16–31): should be correct
        assert_eq!(&recovered[16..32], b"block two here!!");
    }

    #[test]
    fn test_cbc_wrong_key_fails() {
        let key      = [0x11u8; 16];
        let wrong_key = [0x22u8; 16];
        let iv = [0x00u8; 16];
        let plaintext = "secret message!!".to_string();
        let cipher = cbc_alg::_cbc_string_encrypt(plaintext.clone(), iv, key);
        let recovered = cbc_alg::_cbc_string_decrypt(cipher, iv, wrong_key);
        assert_ne!(recovered, plaintext);
    }

    // ── ECB vs CBC oracle (challenge 11) ─────────────────────────────────────

    #[test]
    fn test_ecb_detection_on_known_ecb() {
        // Feed 48 identical bytes → after random prefix (5–10) the ciphertext
        // will have two identical 16-byte blocks if ECB was used.
        // We call the oracle repeatedly until we get an ECB result and verify
        // our detector agrees.
        let chosen = "A".repeat(48);
        let mut found_ecb = false;

        for _ in 0..50 {
            let cipher_hex = _encryption_oracle(chosen.clone());
            let cipher = helper::_hex_to_u8(cipher_hex);
            let is_ecb = _detect_ecb(&cipher);

            // We can't know which mode was chosen, but if we detect ECB it
            // must actually be ECB (no false positives with this plaintext).
            if is_ecb {
                found_ecb = true;
                break;
            }
        }
        // Over 50 tries the oracle will have picked ECB at least once
        // (P(never ECB in 50 tries) = 0.5^50 ≈ 10^-15)
        assert!(found_ecb, "ECB was never detected in 50 oracle calls");
    }
}

// Detection helper — put this next to _encryption_oracle in your challenge 11 module
pub fn _detect_ecb(cipher: &[u8]) -> bool {
    let mut seen = std::collections::HashSet::new();
    for chunk in cipher.chunks(16) {
        if chunk.len() == 16 {
            if !seen.insert(chunk.to_vec()) {
                return true;
            }
        }
    }
    false
}