// use std::mem::transmute;

fn concat(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    a.iter().copied().chain(b.iter().copied()).collect()
}

pub fn enc_u32(value: u32) -> Vec<u8> {
    value.to_be_bytes().to_vec()
}

// pub fn enc_f64(value: f64) -> Vec<u8> {
//     value.to_le_bytes().to_vec()
// }

pub fn enc_property(value: String) -> Vec<u8> {
    let mut buffer = value.into_bytes();
    buffer.push(super::TERMINATOR);
    buffer
}

pub fn enc_string(value: String) -> Vec<u8> {
    let tail = enc_property(value);
    let head = enc_u32(tail.len() as u32);
    concat(head, tail)
}

pub fn enc_boolean(value: bool) -> Vec<u8> {
    if value {
        vec![0x01]
    } else {
        vec![0x00]
    }
}
