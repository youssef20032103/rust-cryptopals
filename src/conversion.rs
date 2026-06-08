use std::{collections::HashSet, fs};


//Challenge 1
pub fn _ascii_to_hex(a: u8) -> u8{    
    if a < 'a' as u8 {
        return a - '0' as u8;
    }
    else {
        return a - 'a' as u8 + 10;
    }
}

pub fn _decode_from_hex(code : String) -> Vec<u8>{
    let mut res : Vec<u8> = Vec::new();
    let mut code = code;
    if code.len() % 2 == 1{
        code = format!("0{}", code);
    }
    let bytes = code.as_bytes();
    for x in (0..code.len()).step_by(2){
        let y: u8 = _ascii_to_hex(bytes[x]) * 16 + _ascii_to_hex(bytes[x+1]);
        res.push(y);
    }
    res   
}

pub fn _convert(input : String) -> String{
    let code: Vec<u8> = _decode_from_hex(input);
    let v: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().collect();
    let mut res: String = String::new();
    let rest = code.len()%3;
    let b: u32 = 63;
    let p = code.len() - rest;
    for x in (0..p).step_by(3){
        let temp : u32 = (code[x] as u32)<<16  | (code[x+1] as u32)<<8 | (code[x+2] as u32);
        let b1 = v[((temp >> 18) & b) as usize]; 
        let b2 = v[((temp >> 12) & b) as usize]; 
        let b3 = v[((temp >> 6) & b) as usize]; 
        let b4 = v[(temp  & b) as usize]; 
        res.push(b1);
        res.push(b2);
        res.push(b3);
        res.push(b4);
    }
    if rest ==1 {
        let temp : u32 = (code[p] as u32)<<16  | (0 as u32)<<8 | (0 as u32);
        let b1 = v[((temp >> 18) & b) as usize]; 
        let b2 = v[((temp >> 12) & b) as usize];  
        res.push(b1);
        res.push(b2);
        res.push('=');
        res.push('=');
    }
    if rest == 2 {
        let temp : u32 = (code[p] as u32)<<16  | (code[p+1] as u32)<<8 | (0 as u32);
        let b1 = v[((temp >> 18) & b) as usize]; 
        let b2 = v[((temp >> 12) & b) as usize]; 
        let b3 = v[((temp >> 6) & b) as usize];  
        res.push(b1);
        res.push(b2);
        res.push(b3);
        res.push('=');
    }
    
    res
}

//Challenge 2
pub fn _xor(a : String, b : String) -> String{
    let hex : Vec<char> = "0123456789abcdef".chars().collect();
    let b1: Vec<u8> = _decode_from_hex(a);
    let b2: Vec<u8> = _decode_from_hex(b);
    let mut b : String = String::new();
    for i in 0..b1.len(){
        let byte = b1[i]^b2[i];
        let high = hex[(byte >> 4) as usize];
        let low  = hex[(byte & 15) as usize];
        b.push(high);
        b.push(low);
    }
    b
}

//Challenge 3
struct ResultSet{
     score: i32,
     text : String,
     key: char
}


pub fn _single_xor(l: Vec<u8>) -> ResultSet{
    let freq : Vec<char> = "etaoin shrdlu".chars().collect();

    let mut max = 0;
    let mut res = String::new();
    let mut ch = ' ';
    for i in 0..256{
        let mut text = String::new();
        let mut m = 0;
        for &j in &l {
            let c = (i as u8)^j;
            if c.is_ascii(){
                text.push(c as char);
                if freq.contains(&(c as char)){
                    m+= 1;
                }
            }   
        }
        if m >= max{
            res = text;
            max = m;
            ch = (i as u8)as char;
        } 
    }
    ResultSet{score: max, text: res, key:ch}
}

//Challenge 4
pub fn _file_xor()-> String{
    let content = fs::read_to_string("challenge4.txt").unwrap();
    let mut corr_string = String::new();
    let mut max = 0;
    for l in content.lines(){
        let line = l.to_string();
        let raw = _decode_from_hex(line);
        let result_set = _single_xor(raw);
        if result_set.score > max {
            max = result_set.score;
            corr_string = result_set.text;
        }
    }
    corr_string
}

//Challenge 5
pub fn _cycle_xor(code : String) -> String{ 
    let hex : Vec<char> = "0123456789abcdef".chars().collect();
    let text : Vec<char> = code.chars().collect(); 
    let key : Vec<char> = "ICE".chars().collect(); 
    let mut res : String= String::new(); 
    let mut counter = 0; 
    for i in text{ 
        let c = (i as u8) ^ (key[counter] as u8); counter = (counter +1)%3; 
        let lower = (c & 0b1111) as usize;
        let higher = (c >> 4) as usize;
        res.push(hex[higher] );
        res.push(hex[lower]);
    } 
    res 
}

//challenge 6
fn _popcount(a : u8)-> i32{
    let mut count = 0;
    let mut byte = a;
    for _ in 0..8{
        if byte & 0b1 == 1{
            count += 1;
        }   
        byte = byte >> 1;
    }
    count
}
fn _hamm_dis_u8(a: Vec<u8>, b:Vec<u8>)-> i32{
    let mut distance = 0;
    for i in 0..a.len(){
        let p = a[i] ^ b[i];
        distance += _popcount(p);
    }
    distance
}
pub fn _hamm_dis(a: String, b:String)-> i32{
    let b1: Vec<u8> = a.into_bytes();
    let b2: Vec<u8> = b.into_bytes();
    _hamm_dis_u8(b1,b2)
}

fn _base64_value(x: char)-> u8{
    let hex = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    for i in 0..hex.len(){
        if hex[i] == (x as u8){
            return i as u8;
        }
    }
    panic!("invalid base64 char");
}
pub fn _base64_decode(content : String) -> Vec<u8>{
    let mut res = Vec::new();
    let mut group: Vec<u8> = Vec::new();
    let mut pad = 0;
    
    for i in content.chars(){
        if i == '='{
            if group.len() <4{
                group.push(0);
                pad += 1;
                continue;
            }
            else { break;}
        }

        let val = _base64_value(i);
        if group.len() >= 4{
            let b1 = group[0] << 2 | group[1] >>4 ;
            let b2 = group[1] <<4 | group[2] >>2 ;
            let b3 = group[2] <<6 | group[3] ;
            res.push(b1);
            res.push(b2);
            res.push(b3);
            group.clear();
        } 
        group.push(val);
    }
    if pad == 1{
        let b1 = group[0] << 2 | group[1] >>4 ;
        let b2 = group[1] <<4 | group[2] >>2 ;
            res.push(b1);
            res.push(b2);
    }
    if pad == 2 {
       let b1 = group[0] << 2 | group[1] >>4 ;
        res.push(b1);
    }
    if (group.len() > 0) & (pad == 0){
        let b1 = group[0] << 2 | group[1] >>4 ;
        let b2 = group[1] <<4 | group[2] >>2 ;
        let b3 = group[2] <<6 | group[3] ;
        res.push(b1);
        res.push(b2);
        res.push(b3);
        group.clear();
    }
    res
}


pub fn _avg_dis(key: i32)-> f64{
    let content = fs::read_to_string("challenge6.txt").unwrap();
    let content = content.replace("\r\n", "").replace("\n", "");
    let byte_list = _base64_decode(content);
    let chunks : Vec<&[u8]> = byte_list.chunks(key as usize).collect();
    let mut res :f64=  0.0;
    for i in (0..6).step_by(2){
        let norm_dis = (_hamm_dis_u8(chunks[i].to_vec(), chunks[i+1].to_vec()) as f64)/((key*5) as f64);
        res += norm_dis as f64;
    }
    res
}
pub fn _avg(low: i32, high: i32, step: usize) -> Vec<f64>{
    let mut res : Vec<f64> = Vec::new();
    for i in (low..high).step_by(step){
        res.push(_avg_dis(i));
    }
    res
}

pub fn _transpose()-> String{
    let content = fs::read_to_string("challenge6.txt").unwrap();
    let content = content.replace("\r\n", "").replace("\n", "");
    let byte_list = _base64_decode(content);
    let chunks : Vec<&[u8]> = byte_list.chunks(29).collect();
    let mut blocks: Vec<Vec<u8>> = vec![Vec::new(); 29];
    let mut res = String::new();
    for i in 0..30{
        for j in 0..chunks.len(){
            if i < chunks[j].len(){
                blocks[i].push(chunks[j][i]);
            }
            else {
                break;
            }
        }
    }
    for i in blocks{
        let c = _single_xor(i).key;
        res.push(c);
    }
    res
}

pub fn _decrypt_repeating_xor(ciphertext: Vec<u8>, key: String) -> String {
    let key_bytes = key.as_bytes();
    let mut res = String::new();
    for i in 0..ciphertext.len() {
        let c = ciphertext[i] ^ key_bytes[i % key_bytes.len()];
        res.push(c as char);
    }
    res
}




// challenge 8
pub fn _partial_decrypt()-> Vec<u8>{
    let content = fs::read_to_string("Challenge8.txt")
        .expect("Could not read file");

    let content = content
    .replace('\n', "")
    .replace('\r', "")
    .replace(' ', "");
    let _text = _base64_decode(content);
    let _block = "8a10247f90d0a05538888ad6205882196f5f6d05c21ec8dca0cb0be02c3f8b09e382963f443aa514daa501257b09a36bf8c4c392d8ca1bf4395f0d5f2542148c7e5ff22237969874bf66cb85357ef99956accf13ba1af36ca7a91a50533c4d89b7353f908c5a166774293b0bf6247391df69c87dacc4125a99ec417221b58170e633381e3847c6b1c28dda2913c011e13fc4406f8fe73bbf78e803e1d995ce4d"
                        .to_string();
    let _block = _block
    .replace('\n', "")
    .replace('\r', "")
    .replace(' ', "");
    let len = _block.len();

    let mut _score =0;
    let mut _aes_text : Vec<u8> = Vec::new();

    for (_, line) in _text.chunks(len).enumerate(){
        let mut set: HashSet<[u8; 16]> = HashSet::new();
        let mut _score = 0;

        for chunk in line.chunks(16) {
            let block: [u8; 16] = chunk.try_into().unwrap();
            if !set.insert(block) {
                _score += 1;
            }
        }
    }
    _aes_text
}
