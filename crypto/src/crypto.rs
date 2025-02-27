use std::u8;

use crate::bytestring::*;

const SBOX: [u8; 256] = [
    0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76,
    0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0,
    0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15,
    0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75,
    0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84,
    0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF,
    0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8,
    0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2,
    0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73,
    0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB,
    0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79,
    0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08,
    0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A,
    0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E,
    0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF,
    0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16];

const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB,
    0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB,
    0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E,
    0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25,
    0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92,
    0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84,
    0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06,
    0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02, 0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B,
    0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73,
    0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E,
    0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B,
    0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4,
    0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F,
    0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF,
    0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D];

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

/* AES helpe functions */
fn aes_construct_chunk(input: &Vec<u8>) -> [[u8; 4]; 4] {
    let mut output = [[0u8; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            output[r][c] = input[4 * c + r];
        }
    }
    output
}

fn aes_deconstruct_chunk(input: &[[u8; 4]; 4]) -> Vec<u8> {
    let mut output = Vec::new();
    for c in 0..4 {
        for r in 0..4 {
            output.push(input[r][c]);
        }
    }
    output
}

fn aes_add_chunks(left: &[[u8; 4]; 4], right: &[[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output = [[0u8; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            output[r][c] = left[r][c] ^ right[r][c];
        }
    }
    output
}

fn aes_apply_sbox(input: &[[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output = [[0u8; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            output[r][c] = SBOX[input[r][c] as usize];
        }
    }
    output
}

fn aes_invert_sbox(input: &[[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output = [[0u8; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            output[r][c] = INV_SBOX[input[r][c] as usize];
        }
    }
    output
}

fn aes_shift_rows(input: &[[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output = [[0u8; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            output[r][c] = input[r][(c + r) % 4];
        }
    }
    output
}

fn aes_invert_shift_rows(input: &[[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut output = [[0u8; 4]; 4];
    for r in 0..4 {
        for c in 0..4 {
            output[r][c] = input[r][(4 + c - r) % 4];
        }
    }
    output
}

fn aes_gf_mult(left: u8, right: u8) -> u8 {
    let mut result = 0;
    let mut a = left;
    let mut b = right;

    for _ in 0..8 {
        if b & 1 == 1 {
            result ^= a;
        }

        let shift_overflow = a >= 0x80;
        a = (a & 0x7f) << 1;

        if shift_overflow {
            a ^= 0x1b;
        }

        b >>= 1;
    }

    result
}

fn aes_mix_column(column: [u8; 4]) -> [u8; 4] {
    let mut output = [0; 4];

    output[0] = aes_gf_mult(2, column[0]) ^ aes_gf_mult(3, column[1]) ^ column[2] ^ column[3];
    output[1] = column[0] ^ aes_gf_mult(2, column[1]) ^ aes_gf_mult(3, column[2]) ^ column[3];
    output[2] = column[0] ^ column[1] ^ aes_gf_mult(2, column[2]) ^ aes_gf_mult(3, column[3]);
    output[3] = aes_gf_mult(3, column[0]) ^ column[1] ^ column[2] ^ aes_gf_mult(2, column[3]);

    output
}

fn aes_unmix_column(column: [u8; 4]) -> [u8; 4] {
    let mut output = [0; 4];

    output[0] = aes_gf_mult(0x0e, column[0]) ^ aes_gf_mult(0x0b, column[1]) ^ aes_gf_mult(0x0d, column[2]) ^ aes_gf_mult(0x09, column[3]);
    output[1] = aes_gf_mult(0x09, column[0]) ^ aes_gf_mult(0x0e, column[1]) ^ aes_gf_mult(0x0b, column[2]) ^ aes_gf_mult(0x0d, column[3]);
    output[2] = aes_gf_mult(0x0d, column[0]) ^ aes_gf_mult(0x09, column[1]) ^ aes_gf_mult(0x0e, column[2]) ^ aes_gf_mult(0x0b, column[3]);
    output[3] = aes_gf_mult(0x0b, column[0]) ^ aes_gf_mult(0x0d, column[1]) ^ aes_gf_mult(0x09, column[2]) ^ aes_gf_mult(0x0e, column[3]);

    output
}

fn aes_mix_columns(i: &[[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut o = [[0; 4]; 4];

    for j in 0..4 {
        [o[0][j], o[1][j], o[2][j], o[3][j]] = aes_mix_column([i[0][j], i[1][j], i[2][j], i[3][j]]);
    }

    o
}

fn aes_unmix_columns(i: &[[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut o = [[0; 4]; 4];

    for j in 0..4 {
        [o[0][j], o[1][j], o[2][j], o[3][j]] = aes_unmix_column([i[0][j], i[1][j], i[2][j], i[3][j]]);
    }

    o
}

fn aes_rotword(i: [u8; 4]) -> [u8; 4] {
    [i[1], i[2], i[3], i[0]]
}

fn aes_subword(i: [u8; 4]) -> [u8; 4] {
    [SBOX[i[0] as usize], SBOX[i[1] as usize], SBOX[i[2] as usize], SBOX[i[3] as usize]]
}

fn aes_next_keyround(key: &[[u8; 4]; 4], round: u8) -> [[u8; 4]; 4] {
    let mut output = [[0; 4]; 4];
    let mut last_column = [key[0][3], key[1][3], key[2][3], key[3][3]];
    last_column = aes_rotword(last_column);
    last_column = aes_subword(last_column);
    let rcon_add;
    if round <= 7 {
        rcon_add = 1 << round;
    } else {
        rcon_add = 0x1b << (round - 8);
    }
    last_column[0] ^= rcon_add;
    for r in 0..4 {
        output[r][0] = last_column[r] ^ key[r][0];
    }

    for c in 1..4 {
        for r in 0..4 {
            output[r][c] = key[r][c] ^ output[r][c-1];
        }
    }

    output
}

fn aes_128_encode_chunk(input: &[[u8; 4]; 4], keys: &[[[u8; 4]; 4]; 11]) -> [[u8; 4]; 4] {
    let mut chunk = aes_add_chunks(&input, &keys[0]);

    for i in 0..9 {
        chunk = aes_apply_sbox(&chunk);
        chunk = aes_shift_rows(&chunk);
        chunk = aes_mix_columns(&chunk);

        chunk = aes_add_chunks(&chunk, &keys[i+1]);
    }

    chunk = aes_apply_sbox(&chunk);
    chunk = aes_shift_rows(&chunk);
    chunk = aes_add_chunks(&chunk, &keys[10]);

    chunk
}

fn aes_128_decode_chunk(input: &[[u8; 4]; 4], keys: &[[[u8; 4]; 4]; 11]) -> [[u8; 4]; 4] {
    let mut chunk = aes_add_chunks(&input, &keys[10]);

    for i in 0..9 {
        chunk = aes_invert_shift_rows(&chunk);
        chunk = aes_invert_sbox(&chunk);
        chunk = aes_add_chunks(&chunk, &keys[9 - i]);
        chunk = aes_unmix_columns(&chunk);
    }

    chunk = aes_invert_shift_rows(&chunk);
    chunk = aes_invert_sbox(&chunk);
    chunk = aes_add_chunks(&chunk, &keys[0]);

    chunk
}

pub fn aes_128_ecb_encode(input: &Vec<u8>, key: &Vec<u8>) -> Option<Vec<u8>> {   
    
    if input.len() % 16 != 0 {
        return None;
    }

    let mut keys = [[[0u8; 4]; 4]; 11];
    keys[0] = aes_construct_chunk(key);
    for i in 1..11 {
        keys[i] = aes_next_keyround(&keys[i-1], (i-1) as u8);
    }

    let mut output = Vec::new();

    let mut remainder = input.clone();
    while remainder.len() > 0 {
        let temp = remainder.split_off(16);
        let mut chunk = aes_construct_chunk(&remainder);
        remainder = temp;

        chunk = aes_128_encode_chunk(&chunk, &keys);

        output.append(&mut aes_deconstruct_chunk(&chunk));
    }
    Some(output)
}

pub fn aes_128_ecb_decode(input: &Vec<u8>, key: &Vec<u8>) -> Option<Vec<u8>> {
    if input.len() % 16 != 0 {
        return None;
    }

    let mut keys = [[[0u8; 4]; 4]; 11];
    keys[0] = aes_construct_chunk(key);
    for i in 1..11 {
        keys[i] = aes_next_keyround(&keys[i-1], (i - 1) as u8);
    }
    let mut output = Vec::new();

    let mut remainder = input.clone();
    while remainder.len() > 0 {
        let temp = remainder.split_off(16);
        let mut chunk = aes_construct_chunk(&remainder);
        remainder = temp;

        chunk = aes_128_decode_chunk(&chunk, &keys);

        output.append(&mut aes_deconstruct_chunk(&chunk));
    }
    Some(output)
}

pub fn aes_128_cbc_encode(input: &Vec<u8>, key: &Vec<u8>, iv: &Vec<u8>) -> Option<Vec<u8>> {
    if input.len() % 16 != 0 {
        return None;
    }

    let mut keys = [[[0u8; 4]; 4]; 11];
    keys[0] = aes_construct_chunk(key);
    for i in 1..11 {
        keys[i] = aes_next_keyround(&keys[i-1], (i-1) as u8);
    }

    let mut output = Vec::new();
    let mut last_chunk = aes_construct_chunk(iv);

    let mut remainder = input.clone();
    while remainder.len() > 0 {
        let temp = remainder.split_off(16);
        let mut chunk = aes_construct_chunk(&remainder);
        remainder = temp;

        chunk = aes_add_chunks(&chunk, &last_chunk);
        chunk = aes_128_encode_chunk(&chunk, &keys);
        last_chunk = chunk;

        output.append(&mut aes_deconstruct_chunk(&chunk));
    }
    Some(output)
}

pub fn aes_128_cbc_decode(input: &Vec<u8>, key: &Vec<u8>, iv: &Vec<u8>) -> Option<Vec<u8>> {
    if input.len() % 16 != 0 {
        return None;
    }

    let mut keys = [[[0u8; 4]; 4]; 11];
    keys[0] = aes_construct_chunk(key);
    for i in 1..11 {
        keys[i] = aes_next_keyround(&keys[i-1], (i - 1) as u8);
    }

    let mut output = Vec::new();
    let mut last_chunk = aes_construct_chunk(iv);

    let mut remainder = input.clone();
    while remainder.len() > 0 {
        let temp = remainder.split_off(16);
        let mut chunk = aes_construct_chunk(&remainder);
        remainder = temp;

        let temp = chunk;
        chunk = aes_128_decode_chunk(&chunk, &keys);
        chunk = aes_add_chunks(&chunk, &last_chunk);
        last_chunk = temp;

        output.append(&mut aes_deconstruct_chunk(&chunk));
    }
    Some(output)
}

pub fn aes_128_ctr_encode(input: &Vec<u8>, key: &Vec<u8>, nonce: &Vec<u8>) -> Option<Vec<u8>> {
    fn counter_to_bytes(counter: u64) -> Vec<u8> {
        //Convert a 64 bit counter to 8 little-endian bytes
        let mut output = Vec::new();
        let mut remainder = counter;
        while output.len() < 8 {
            output.push((remainder & 0xff) as u8);
            remainder >>= 8;
        }
        output
    }

    if nonce.len() != 8 {
        println!("Incorrect nonce length {:?}", nonce.len());
        return None;
    }

    let mut counter = 0u64;
    let mut key_generator = Vec::new();
    while key_generator.len() <= input.len() {
        key_generator.append(&mut nonce.clone());
        key_generator.append(&mut counter_to_bytes(counter));
        counter += 1;
    }
    let mut keystream = aes_128_ecb_encode(&key_generator, &key)?;
    keystream.split_off(input.len());
    Some(xor(&input, &keystream))
}

pub fn aes_128_ctr_decode(input: &Vec<u8>, key: &Vec<u8>, nonce: &Vec<u8>) -> Option<Vec<u8>> {
    aes_128_ctr_encode(&input, &key, &nonce)
}