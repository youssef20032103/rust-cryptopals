
const _HEX: &[u8;16] = b"0123456789abcdef";

fn _ascii_hex_value(y: u8) -> u8 {
    if y >= b'a' {
        y - b'a' + 10
    } else if y >= b'A' {
        y - b'A' + 10
    } else {
        y - b'0'
    }
}

pub fn _u8_to_hex(x : Vec<u8>) -> String {
    let mut res = String::new();
    for element in x{
        res.push(_HEX[(element >> 4) as usize] as char);
        res.push(_HEX[(element & 0x0f) as usize] as char)
    }
    res
}
pub fn _hex_to_u8(x : String) -> Vec<u8>{
    let mut res : Vec<u8> = Vec::new();
    let mut code = x;
    if code.len() % 2 == 1{
        code = format!("0{}", code);
    }
    let bytes = code.as_bytes();
    for x in (0..code.len()).step_by(2){
        let y: u8 = _ascii_hex_value(bytes[x]) * 16 + _ascii_hex_value(bytes[x+1]);
        res.push(y);
    }
    res   
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
pub fn _base64_to_u8(content : String) -> Vec<u8>{
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

