


mod aes_alg;
mod conversion;
mod set2;
mod cbc_alg;
mod helper;

 
fn main() {
    
    let x = b"YELLOW SUBMARINE".to_vec();

    let padded = aes_alg::_pkcs7_padding(x.clone(), 16);
    let unpadded = aes_alg::_pksc7_unpadding(padded);

    assert_eq!(x, unpadded);
}