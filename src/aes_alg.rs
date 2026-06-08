use std::fs;

use crate::{conversion, helper};

//-----------------------------------Helpers---------------------------------------------------------------------------
//GF table multiplication
fn _xtime(t: u8) -> u8{
    if t & 0b1000_0000 != 0 {
        (t <<1) ^ 0x1b 
    } else {
        t << 1 
    }
} 
fn _xtime3(t: u8)-> u8{
    _xtime(t) ^ t
}
fn _xtime9(t: u8) -> u8 {
    let x2 = _xtime(t);
    let x4 = _xtime(x2);
    let x8 = _xtime(x4);
    x8 ^ t
}
fn _xtime11(t: u8) -> u8 {
    let x2 = _xtime(t);
    let x4 = _xtime(x2);
    let x8 = _xtime(x4);
    x8 ^ x2 ^ t
}
fn _xtime13(t: u8) -> u8 {
    let x2 = _xtime(t);
    let x4 = _xtime(x2);
    let x8 = _xtime(x4);
    x8 ^ x4 ^ t
}
fn _xtime14(t: u8) -> u8 {
    let x2 = _xtime(t);
    let x4 = _xtime(x2);
    let x8 = _xtime(x4);
    x8 ^ x4 ^ x2
}
//SBox lookup table 
const _SBOX: [u8; 256] = [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
        0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
        0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
        0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
        0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
        0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
        0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
        0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
        0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
        0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
        0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
        0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
        0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
        0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
        0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
    ];
//Sbox inverse lookuptable
const _INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
    ];
//xor method 
pub fn _xor<const N:usize>(a:&[u8;N], b:&[u8;N]) -> [u8;N]{
    let mut res = [0u8;N];
    for i in 0..N{
        res[i] = a[i] ^ b[i];
    }
    res
}
// Round key (xor with key)
pub fn _round_key(a : [u8;16] , b:[u8;16]) -> [u8;16]{
    _xor(&a,&b)
}
//expand key
fn _rot_word(a:[u8;4]) -> [u8;4]{
    [a[1],a[2],a[3],a[0]]
}
fn _sub_word(a:[u8;4]) -> [u8;4]{
    [_lookup(a[0]),_lookup(a[1]),_lookup(a[2]),_lookup(a[3])]
}
//key round constants 
const _RCON: [[u8; 4]; 10] = [
    [0x01, 0x00, 0x00, 0x00],
    [0x02, 0x00, 0x00, 0x00],
    [0x04, 0x00, 0x00, 0x00],
    [0x08, 0x00, 0x00, 0x00],
    [0x10, 0x00, 0x00, 0x00],
    [0x20, 0x00, 0x00, 0x00],
    [0x40, 0x00, 0x00, 0x00],
    [0x80, 0x00, 0x00, 0x00],
    [0x1B, 0x00, 0x00, 0x00],
    [0x36, 0x00, 0x00, 0x00],
];
//converting text to proper vec<u8>
pub fn _convert_file(file:&str)-> Vec<u8>{
    let content = fs::read_to_string(file)
        .expect("Could not read file");

    let content = content
    .replace('\n', "")
    .replace('\r', "")
    .replace(' ', "");
    let text = conversion::_base64_decode(content);
    text
}
//padding/unpadding
pub fn _pkcs7_padding(mut text : Vec<u8>, size: usize) -> Vec<u8>{
    let _pad_len = size - text.len() %size;
    for _ in 0.._pad_len{
        text.push(_pad_len as u8);
    }
    text
}
pub fn _pksc7_unpadding(mut text: Vec<u8>)-> Vec<u8>{
    let _pad_len  = *text.last().unwrap() as usize;
    text.truncate(text.len() - _pad_len);
    text
}





//-----------------------------------Aes methods---------------------------------------------------------------------------
// Subbyte operation  
fn _lookup(x:u8)-> u8{
    _SBOX[x as usize]
} 
fn _reverse_lookup(x:u8)-> u8{
    _INV_SBOX[x as usize]
} 


pub fn _sub_byte(x : [u8;16]) -> [u8;16]{
    let mut res = [0u8;16];
    for i in 0..16{
        res[i] = _lookup(x[i]);
    }
    println!("Sub byte result: {:02x?}",res);
    res
}
pub fn _reverse_sub_byte(x : [u8;16]) -> [u8;16]{
    let mut res = [0u8;16];
    for i in 0..16{
        res[i] = _reverse_lookup(x[i]);
    }
    println!("Reverse Sub byte result: {:02x?}",res);
    res
}


// Shift row
pub fn _shift_row(chunk : [u8;16]) -> [u8;16]{
    let mut res: [u8;16] = [0u8;16];
    res[0] = chunk[0];   res[4] = chunk[4];   res[8] = chunk[8];   res[12] = chunk[12];
    res[1] = chunk[5];   res[5] = chunk[9];   res[9] = chunk[13];   res[13] = chunk[1];
    res[2] = chunk[10];   res[6] = chunk[14];   res[10] = chunk[2];   res[14] = chunk[6];
    res[3] = chunk[15];   res[7] = chunk[3];   res[11] = chunk[7];   res[15] = chunk[11];
    println!("Shift row result: {:02x?}",res);
    res
}
pub fn _reverse_shift_row(chunk : [u8;16]) -> [u8;16]{
    let mut res: [u8;16] = [0u8;16];
    res[0] = chunk[0];   res[4] = chunk[4];   res[8] = chunk[8];   res[12] = chunk[12];
    res[1] = chunk[13];   res[5] = chunk[1];   res[9] = chunk[5];   res[13] = chunk[9];
    res[2] = chunk[10];   res[6] = chunk[14];   res[10] = chunk[2];   res[14] = chunk[6];
    res[3] = chunk[7];   res[7] = chunk[11];   res[11] = chunk[15];   res[15] = chunk[3];
    println!("reverse Shift row result: {:02x?}",res);
    res
}


// Column shift
fn _mix_one_column(t : [u8;4]) -> [u8;4]{
    let mut res = [0u8;4];
    res[0] = _xtime(t[0]) ^ _xtime3(t[1]) ^ (t[2]) ^ (t[3]);
    res[1] = (t[0]) ^ _xtime(t[1]) ^ _xtime3(t[2]) ^(t[3]);
    res[2] = (t[0]) ^(t[1]) ^_xtime(t[2]) ^ _xtime3(t[3]);
    res[3] = _xtime3(t[0]) ^(t[1])^(t[2])^ _xtime(t[3]);

    res
}
pub fn _mix_column(slice : [u8;16])-> [u8;16]{
    let mut res = [0u8;16];
    for (index, chunk) in slice.chunks(4).enumerate(){
        let column = _mix_one_column(chunk.try_into().unwrap());
        for i in 0..4{
            res[index*4 + i] = column[i];
        } 
    }
    println!("Column mix result: {:02x?}",res);
    res
}

fn _reverse_mix_one_column(t : [u8;4]) -> [u8;4]{
    let mut res = [0u8;4];
    res[0] = _xtime14(t[0]) ^ _xtime11(t[1]) ^ _xtime13(t[2]) ^ _xtime9(t[3]);
    res[1] = _xtime9(t[0])  ^ _xtime14(t[1]) ^ _xtime11(t[2]) ^ _xtime13(t[3]);
    res[2] = _xtime13(t[0]) ^ _xtime9(t[1])  ^ _xtime14(t[2]) ^ _xtime11(t[3]);
    res[3] = _xtime11(t[0]) ^ _xtime13(t[1]) ^ _xtime9(t[2])  ^ _xtime14(t[3]);
    res
}
pub fn _reverse_mix_column(slice : [u8;16])-> [u8;16]{
    let mut res = [0u8;16];
    for (index, chunk) in slice.chunks(4).enumerate(){
        let column = _reverse_mix_one_column(chunk.try_into().unwrap());
        for i in 0..4{
            res[index*4 + i] = column[i];
        } 
    }
    println!("Reverse Column mix result: {:02x?}",res);
    res
}

//generate key
pub fn _generate_key(a:[u8;16])-> [[u8;16];11]{
    let mut flat_key = [[0u8;4];4];
    for (index,chunk) in a.chunks(4).enumerate(){
        for i in 0..4{
            flat_key[index][i] = chunk[i];
        }
    }

    let mut res : [[u8;4];44] = [[0u8;4];44];
    let mut key : [[u8;16];11] = [[0u8;16];11];
    for i in 0..4{
        res[i] = flat_key[i];                                                              
    }
    for i in 4..44{
        let mut temp = res[i-1];
        if i % 4 ==0 {
            temp = _rot_word(temp);
            temp = _sub_word(temp);
            temp = _xor(&temp,&_RCON[i/4-1]);
        }
        res[i] = _xor(&res[i-4],&temp);
    }
    

    for round in 0..11 {
        for col in 0..4 {
            for row in 0..4 {
                key[round][col*4 + row]
                    = res[round*4 + col][row];
            }
        }
    }
    println!("{:02x?}",key[0]);
    println!("{:02x?}",key[10]);
    key
}


//-----------------------------------Aes algorithm---------------------------------------------------------------------------
pub fn _encrypt_block(block: &[u8], key : [[u8;16];11])-> [u8;16]{
    let mut state :[u8;16] = block.try_into().unwrap();
        state = _xor(&state,&key[0]);
        for i in 1..10{
            state = _sub_byte(state);
            state = _shift_row(state);
            state = _mix_column(state);
            state = _xor(&state,&key[i]);
        }
        state = _sub_byte(state);
        state = _shift_row(state);
        state = _xor(&state,&key[10]);
        state
}
pub fn _decrypt_block(block: &[u8], key : [[u8;16];11])-> [u8;16]{
    let mut state :[u8;16] = block.try_into().unwrap();
    state = _xor(&state,&key[10]);
    state = _reverse_shift_row(state);
    state = _reverse_sub_byte(state);

    for i in 1..10{
        state = _xor(&state,&key[10-i]);
        state = _reverse_mix_column(state);
        state = _reverse_shift_row(state);
        state = _reverse_sub_byte(state);
    }

    state = _xor(&state,&key[0]);
    state
}

pub fn _encrypt(text : Vec<u8>, key : [u8;16]) -> Vec<u8>{
    let _expanded_key = _generate_key(key);
    let mut res : Vec<u8> = Vec::new();
    for (_index, chunk) in text.chunks(16).enumerate(){
        let state = _encrypt_block(chunk, _expanded_key);
        res.extend(state);
    }
    res
}
pub fn _decrypt(text : Vec<u8>, key : [u8;16]) -> Vec<u8>{
    let _expanded_key = _generate_key(key);
    let mut res : Vec<u8> = Vec::new();
    for (_index, chunk) in text.chunks(16).enumerate(){
        let state = _decrypt_block(chunk, _expanded_key);
        res.extend(state);
    }
    res
}

pub fn _aes_encrypt(text: Vec<u8>, key:[u8;16])-> String{
    let content = _pkcs7_padding(text, 16);
    let result = _encrypt(content, key);
    helper::_u8_to_hex(result)
}
pub fn _aes_decrypt(text: Vec<u8>, key: [u8;16]) -> String{
    let mut result = _decrypt(text, key);
    result = _pksc7_unpadding(result);
    String::from_utf8(result).unwrap()
}


pub fn _aes_string_encrypt(text: String, key:[u8;16])-> String{
    let content = text.as_bytes().to_vec();
    _aes_encrypt(content, key)
}

pub fn _aes_string_decrypt(text: String, key: [u8;16]) -> String{
    let content = helper::_hex_to_u8(text);
    _aes_decrypt(content, key)
}










#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_known_vector() {
        let key = [
            0x00,0x01,0x02,0x03,
            0x04,0x05,0x06,0x07,
            0x08,0x09,0x0a,0x0b,
            0x0c,0x0d,0x0e,0x0f,
        ];

        let plaintext = vec![
            0x00,0x11,0x22,0x33,
            0x44,0x55,0x66,0x77,
            0x88,0x99,0xaa,0xbb,
            0xcc,0xdd,0xee,0xff,
        ];

        let expected = vec![
            0x69,0xc4,0xe0,0xd8,
            0x6a,0x7b,0x04,0x30,
            0xd8,0xcd,0xb7,0x80,
            0x70,0xb4,0xc5,0x5a,
        ];

        let ciphertext = _encrypt(plaintext, key);

        println!("Expected   : {:02x?}", expected);
        println!("Ciphertext : {:02x?}", ciphertext);

        assert_eq!(ciphertext, expected);
    }
}










