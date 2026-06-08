
const _HEX: &[u8;16] = b"0123456789abcdef";

fn _ascii_hex_value(y: u8)-> u8{
    if y < 'a' as u8{
        return y - '0' as u8;
    }
    else if y < 'A' as u8{
        return y - 'a' as u8 +10 ;
    }
    else {
        return y - 'A' as u8 + 10;
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

