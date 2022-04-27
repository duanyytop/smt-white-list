use hex;
use std::convert::TryInto;

pub fn remove_0x(str: &str) -> &str {
    if str.contains("0x") {
        &str[2..]
    } else {
        str
    }
}

pub fn parse_vec_n<const N: usize>(vec: Vec<u8>) -> [u8; N] {
    vec.try_into().unwrap_or_else(|v: Vec<u8>| {
        panic!("Expected a Vec of length {} but it was {}", N, v.len())
    })
}

pub fn parse_bytes_n<const N: usize>(value: String) -> [u8; N] {
    let vec = hex::decode(value).expect("Hex decode error");
    if vec.len() != N {
        panic!("Expected a Vec of length {} but it was {}", N, vec.len())
    }
    parse_vec_n(vec)
}

pub fn parse_bytes(value: String) -> Vec<u8> {
    let v = remove_0x(&value);
    hex::decode(v).expect("Hex decode error")
}
