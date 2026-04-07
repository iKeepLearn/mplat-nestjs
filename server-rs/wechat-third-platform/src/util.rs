use crate::error::Error;

const BLOCK_SIZE: usize = 32;
pub fn pkcs7_encode(mut data: Vec<u8>) -> Vec<u8> {
    let pad_size = BLOCK_SIZE - (data.len() % BLOCK_SIZE);
    let pad_byte = pad_size as u8;
    data.resize(data.len() + pad_size, pad_byte);
    data
}

pub fn pkcs7_decode(data: &[u8]) -> Result<&[u8], Error> {
    if data.is_empty() {
        return Err(Error::Config("Empty data after decryption".to_string()));
    }
    let pad_size = *data.last().unwrap() as usize;
    if !(1..=BLOCK_SIZE).contains(&pad_size) || pad_size > data.len() {
        return Ok(data); // 未抛出异常，作兼容处理
    }
    Ok(&data[0..data.len() - pad_size])
}
