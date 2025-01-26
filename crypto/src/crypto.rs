use crate::bytestring::xor;

pub fn single_char_xor(input: &Vec<u8>, key: &u8) -> Vec<u8> {
    let mut encoder = Vec::new();
    for _ in 0..input.len() {
        encoder.push(*key);
    }
    xor(&input, &encoder)
}

pub fn repeat_key_xor(input: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let mut encoder = Vec::new();
    for _ in 0..((input.len() / key.len()) + 1) {
        encoder.append(key.clone().as_mut());
    }
    while encoder.len() > input.len() {
        encoder.pop();
    }
    xor(&input, &encoder)
}