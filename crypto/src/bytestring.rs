use rand::prelude::*;

pub fn rand_bytes(length: u8) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let mut rng = rand::rng();

    for _ in 0..length {
        output.push(rng.random());
    }

    output
}

pub fn from_hex(input: &str) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    let mut istr = input.to_string();
    if istr.len() % 2 != 0 {
        istr.insert(0, '0');
    }
    let mut ichars = istr.chars();
    while let (Some(c1), Some(c2)) = (ichars.next(), ichars.next()) {
        let d1: u8 = c1.to_digit(16)? as u8;
        let d2: u8 = c2.to_digit(16)? as u8;
        bytes.push((d1 << 4) + d2);
    }
    Some(bytes)
}

pub fn from_ascii(input: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut ichars = input.chars();
    while let Some(c)= ichars.next() {
        bytes.push(c as u8);
    }
    bytes
}

pub fn from_base64(input: &str) -> Option<Vec<u8>> {
    fn b64_to_b(c: char) -> Option<u8> {
        let char_v = c as u8;
        if char_v >= b'A' && char_v <= b'Z' {
            Some(char_v - b'A')
        } else if char_v >= b'a' && char_v <= b'z' {
            Some(char_v - b'a' + 26)
        } else if char_v >= b'0' && char_v <= b'9' {
            Some(char_v - b'0' + 52)
        } else if c == '+' {
            Some(62)
        } else if c == '/' {
            Some(63)
        } else if c == '=' {
            Some(0)
        } else {
            None
        }
    }

    let mut istr = input.to_string();
    let mut output = Vec::new();
    while istr.len() % 4 != 0 {
        istr.push('=');
    }
    while let (Some(c4), Some(c3), Some(c2), Some(c1)) = (istr.pop(), istr.pop(), istr.pop(), istr.pop()) {
        if c1 == '=' || c2 == '=' {
            println!("Invalid b64: {}", input);
            return None;
        }
        
        let cv1 = b64_to_b(c1)?;
        let cv2 = b64_to_b(c2)?;
        let cv3 = b64_to_b(c3)?;
        let cv4 = b64_to_b(c4)?;
        
        let b1 = (cv1 << 2) + (cv2 >> 4);
        let b2 = ((cv2 & 0xf) << 4) + (cv3 >> 2);
        let b3 = ((cv3 & 0x7) << 6) + cv4;

        output.insert(0, b1);
        if c3 != '=' {
            output.insert(1, b2);
        }
        if c4 != '=' {
            output.insert(2, b3);
        }
    }
    Some(output)
}

pub fn to_hex(input: &Vec<u8>) -> String {
    fn c_to_hex(c: u8) -> char {
        if c <= 9 {
            (b'0' + c) as char
        } else {
            (b'a' + c - 10) as char
        }
    }
    let mut output = String::new();
    if input.len() == 0 {
        output.push('0');
    }
    for &byte in input {
        let b1 = byte >> 4;
        let b2 = byte & 0b00001111;
        output.push(c_to_hex(b1));
        output.push(c_to_hex(b2));
    }
    output
}

pub fn to_base64(input: &Vec<u8>) -> String {
    fn c_to_b64(c: u8) -> char {
        if c <= 25 {
            (b'A' + c) as char
        } else if c <= 51 {
            (b'a' + c - 26) as char
        } else if c <= 61 {
            (b'0' + c - 52) as char
        } else if c == 62 {
            '+'
        } else {
            '/'
        }
    }
    let mut output = String::new();
    if input.len() == 0 {
        output.push('0');
    }
    let mut remainder_size = 0;
    let mut remainder = 0;
    for &byte in input {
        let cb = byte >> (2 + remainder_size);
        output.push(c_to_b64((remainder << (6 - remainder_size)) + cb));
        remainder_size += 2;
        remainder = (byte << (8 - remainder_size)) >> (8 - remainder_size);
        if remainder_size == 6 {
            output.push(c_to_b64(remainder));
            remainder_size = 0;
            remainder = 0;
        }
    }
    if remainder_size > 0 {
        output.push(c_to_b64(remainder << (6 - remainder_size)));
        output.push('=');
        if remainder_size == 2 {
            output.push('=');
        }
    }
    output
}

pub fn to_ascii(input: &Vec<u8>) -> String {
    let mut result = String::new();
    for &c in input {
        result.push(c as char);
    }
    result
}

pub fn xor(input: &Vec<u8>, other: &Vec<u8>) -> Vec<u8> {
    let mut self_padded = input.clone();
    let mut other_padded = other.clone();
    
    if input.len() < other.len() {
        for _ in 0..(other.len() - input.len()) {
            self_padded.insert(0, 0);
        }
    }
    if input.len() > other.len() {
        for _ in 0..(input.len() - other.len()) {
            other_padded.insert(0, 0);
        }
    }

    let mut output = Vec::new();
    for i in 0..(self_padded.len()) {
        output.push(self_padded[i] ^ other_padded[i]);
    }
    output
}

pub fn hamming_dist(input: &Vec<u8>, other: &Vec<u8>) -> u32 {
    fn weight(d: u8) -> u32 {
        let mut w = 0;
        for i in 0..8 {
            if (d & (1 << i)) != 0 {
                w += 1;
            }
        }
        w
    }
    
    let mut sum: u32 = 0;
    let mut iclone = input.clone();
    let mut oclone = other.clone();
    while let (Some(d1), Some(d2)) = (iclone.pop(), oclone.pop()) {
        sum += weight(d1 ^ d2);
    }
    sum
}

pub fn pkcs7pad(input: &Vec<u8>, block_size: u8) -> Vec<u8> {
    let padding_length: u8 = block_size - ((input.len() as u8) % block_size);
    let mut output = input.clone();
    for _ in 0..padding_length {
        output.push(padding_length);
    }
    output
}