#![allow(dead_code)]
mod bytestring;
mod text_analysis;
mod crypto;
use std::{fs::File, i32, io::Read};

use bytestring::*;
use text_analysis::*;
use crypto::*;
use rand::Rng;

fn main() {
    challenge_20();
}

fn challenge_3() {
    const CHALLENGE: &str = "3f1b5a343f034832193b153c482f1705392f021f5f0953290c4c43312b36";
    let input = from_hex(CHALLENGE).expect("Invalid hex string given");
    let mut best_output = "No output found".to_string();
    let mut best_score = f64::MAX;
    for i in 0..=u8::MAX {
        let output = single_char_xor(&input, &i);
        if output.iter().min().unwrap() > &0x1fu8 && output.iter().max().unwrap() < &0x7fu8{
            let output_s = to_ascii(&output);
            let score = char_freq(&output_s);
            if score < best_score {
                best_score = score;
                best_output = output_s;
            }
        }
    }
    println!("Best Guess: {}", best_output);
}

fn challenge_4() {
    let mut file = File::open("./4.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let mut best_guesses = Vec::new();
    for line in contents.split('\n') {
        let mut best_output = "No output found".to_string();
        let mut best_score = f64::MAX;
        let input = from_hex(line).expect("Invalid hex string given");
        for i in 0..=u8::MAX {
            let output = single_char_xor(&input, &i);
            let output_s = to_ascii(&output);
            let score = char_freq(&output_s);
            if score < best_score {
                best_score = score;
                best_output = output_s;
            }
        }
        if best_score != f64::MAX {
            best_guesses.push((best_output, best_score));
        }
    }
    let mut best_output = "No output found".to_string();
    let mut best_score = f64::MAX;
    for (g, s) in best_guesses {
        if s < best_score {
            best_score = s;
            best_output = g;
        }
    }
    println!("Best Guess: {:?}, {:?}", best_output, best_score);
}

fn challenge_5() {
    const CHALLENGE: &str = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    const KEY: &str = "ICE";
    let output = repeat_key_xor(&from_ascii(CHALLENGE), &from_ascii(KEY));
    println!("{:?}", to_hex(&output));
}

fn challenge_6() {
    let mut file = File::open("./6.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let input = from_base64(&(contents.replace("\n", ""))).expect("Invalid base64 given");

    // let mut scores = Vec::new();

    // for keysize in 1..40 {
    //     let mut tests = Vec::new();
    //     for i in 0..8 {
    //         tests.push(input[(i*keysize)..((i+1) * keysize)].to_vec());
    //     }
    //     let mut score  = 0.0;
    //     for test1 in tests.clone() {
    //         for test2 in tests.clone() {
    //             score += hamming_dist(&test1, &test2) as f64
    //         }
    //     }
    //     scores.push((keysize, score / (keysize as f64)));
    // }

    // scores.sort_by(|a, b| (a.1).total_cmp(&b.1));

    // println!("{:?}", scores);

    /* [(29, 153.51724137931035), (1, 156.0), (3, 167.33333333333334), (5, 172.8), (4, 175.0), (34, 175.76470588235293), (12, 176.83333333333334), (24, 178.25), (8, 178.5), (13, 178.6153846153846), (15, 178.66666666666666), (14, 179.42857142857142), (17, 180.11764705882354), (33, 180.3030303030303), (37, 180.32432432432432), (10, 180.4), (16, 180.5), (21, 180.95238095238096), (22, 181.0), (9, 181.11111111111111), (28, 181.35714285714286), (32, 181.5625), (30, 182.06666666666666), (31, 182.19354838709677), (23, 182.43478260869566), (20, 182.6), (18, 182.77777777777777), (36, 182.77777777777777), (19, 182.8421052631579), (35, 182.85714285714286), (38, 183.0), (7, 183.71428571428572), (25, 183.76), (26, 184.84615384615384), (11, 185.0909090909091), (39, 185.3846153846154), (27, 186.8148148148148), (6, 187.0), (2, 188.0)] */

    const KEYSIZE: usize = 29;

    let mut blocks = Vec::new();

    for j in 0..KEYSIZE {
        let mut block = Vec::new();
        for i in 0..(input.len() / KEYSIZE) {
            block.push(*input.get(i * KEYSIZE + j).expect("Invalid index"));
        }
        blocks.push(block);
    }

    let mut guessed_key = Vec::new();
    for block in blocks {
        let mut best_key: u8 = 0;
        let mut best_score = f64::MAX;
        for i in 32..=126 {
            let output = single_char_xor(&block, &i);
            let output_s = to_ascii(&output);
            let score = char_freq(&output_s);
            if score < best_score {
                best_score = score;
                best_key = i;
            }
        }
        guessed_key.push(best_key);
    }

    let output = repeat_key_xor(&input, &guessed_key);

    println!("{}", to_ascii(&output));
    println!("{:?}", to_ascii(&guessed_key));
}

fn challenge_7() {
    let mut file = File::open("./7.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let input = from_base64(&(contents.replace("\n", ""))).expect("Invalid base64 given");
    let key = from_ascii("YELLOW SUBMARINE");
    let plaintext = aes_128_ecb_decode(&input, &key).expect("Decoding failed");
    println!("{}", to_ascii(&plaintext));
}

fn challenge_8() {
    let mut file = File::open("./8.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    for line in contents.split('\n') {
        let mut seen_strings = Vec::new();
        let mut remainder = line.to_string();
        while remainder.len() > 0 {
            let temp = remainder.split_off(32);
            if seen_strings.contains(&remainder) {
                println!("Candidate {} has duplicate block {}", line, remainder);
            }
            seen_strings.push(remainder);
            remainder = temp;
        }
    }
}

fn challenge_9() {
    let test = from_ascii("YELLOW SUBMARINE");
    println!("{:?}", to_ascii(&pkcs7pad(&test, 16)));
}

fn challenge_10() {
    let mut file = File::open("./10.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let input = from_base64(&(contents.replace("\n", ""))).expect("Invalid base64 given");
    let key = from_ascii("YELLOW SUBMARINE");
    let iv = from_ascii("\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
    let plaintext = aes_128_cbc_decode(&input, &key, &iv).expect("Decoding failed");
    println!("{}", to_ascii(&plaintext));
}

fn challenge_11() {
    fn enc_oracle(input: &Vec<u8>) -> (bool, Vec<u8>) {
        let mut rng = rand::rng();
        let mut plaintext = rand_bytes(rng.random_range(5..=10));
        plaintext.append(&mut input.clone());
        plaintext.append(&mut rand_bytes(rng.random_range(5..=10)));
        
        plaintext = pkcs7pad(&plaintext, 16);

        let key  = rand_bytes(16);

        if rng.random_range(0..=1) == 0 {
            let iv = rand_bytes(16);
            (true, aes_128_cbc_encode(&plaintext, &key, &iv).unwrap())
        } else {
            (false, aes_128_ecb_encode(&plaintext, &key).unwrap())
        }
    }

    let mut correct_guesses = 0;
    let test_data = from_ascii("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    for _ in 0..100 {
        let (correct_answer, ciphertext) = enc_oracle(&test_data);

        //check if the second and third block of the ciphertext (both encoding 16 'A's) are the same
        let mut guess = false;
        for i in 0..16 {
            guess |= ciphertext[16+i] != ciphertext[32+i];
        }
        if correct_answer == guess {
            correct_guesses += 1;
        }
    }

    println!("Correctly guess {:?}/100 times", correct_guesses);
}

fn challenge_12() {
    fn enc_oracle(input: &Vec<u8>) -> Vec<u8> {
        let mut plaintext = input.clone();
        plaintext.append(&mut from_base64("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap());
        plaintext = pkcs7pad(&plaintext, 16);

        //One-time randomly generated key
        let key  = from_base64("QiB1YBiIylHbtl477czO7w==").unwrap();

        aes_128_ecb_encode(&plaintext, &key).unwrap()
    }

    let mut prefix = Vec::new();
    let mut current_len: usize = enc_oracle(&prefix).len();
    while enc_oracle(&prefix).len() == current_len {
        prefix.push(b'A');
    }
    current_len = enc_oracle(&prefix).len();
    let mut block_size = 0;
    while enc_oracle(&prefix).len() == current_len {
        prefix.push(b'A');
        block_size += 1;
    }

    println!("Block size: {:?}", block_size);

    while prefix.len() < 2 * block_size {
        prefix.push(b'A');
    }

    let ciphertext = enc_oracle(&prefix);

    let mut using_ecb = true;
    for i in 0..block_size {
        using_ecb &= ciphertext[i] == ciphertext[block_size+i];
    }

    println!("Using ECB mode: {:?}", using_ecb);

    let mut known_bytes = Vec::new();

    for _ in 0..enc_oracle(&Vec::new()).len() {
        prefix = Vec::new();
        let padding_length = block_size - (known_bytes.len() % block_size) - 1;
        for _ in 0..padding_length {
            prefix.push(b'A');
        }
        let challenge = enc_oracle(&prefix);
        prefix.append(&mut known_bytes.clone());
        let current_block = known_bytes.len() / block_size;
        for b in 0..u8::MAX {
            prefix.push(b);
            let candidate = enc_oracle(&prefix);
            let mut found_byte = true;
            for i in 0..block_size {
                found_byte &= challenge[block_size * current_block + i] == candidate[block_size * current_block + i];
            }
            if found_byte {
                known_bytes.push(b);
                break;
            }
            prefix.pop();
        }
    }

    println!("{:?}", to_ascii(&known_bytes));
}

fn challenge_13() {
    fn profile_for(input: &String) -> Vec<u8> {
        let mut plaintext = String::from("email=");
        plaintext += input;
        plaintext += "&uid=10&role=user";
        let bytes = pkcs7pad(&from_ascii(&plaintext), 16);

        //One-time randomly generated key
        let key  = from_base64("QiB1YBiIylHbtl477czO7w==").unwrap();

        aes_128_ecb_encode(&bytes, &key).unwrap()
    }

    fn decode(ciphertext: Vec<u8>) -> String {
        let key  = from_base64("QiB1YBiIylHbtl477czO7w==").unwrap();
        to_ascii(&aes_128_ecb_decode(&ciphertext, &key).unwrap())
    }

    let c1 = profile_for(&String::from("aaaaaaaaa.admin\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11"));
    let mut c2 = profile_for(&String::from("abc@gmail.com"));
    for _ in 0..16 {
        c2.pop();
    }
    for i in 0..16 {
        c2.push(c1[16+i]);
    }
    println!("{:?}", decode(c2));
}

fn challenge_14() {
    fn enc_oracle(input: &Vec<u8>) -> Vec<u8> {
        let mut plaintext = input.clone();
        plaintext.append(&mut from_base64("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").unwrap());

        //One-time randomly generated key
        let key  = from_base64("QiB1YBiIylHbtl477czO7w==").unwrap();

        //One-time randomly generated prefix
        let mut random_prefix = from_base64("nI9VM1yVfEI0oQ0+qPg+dJ03pZo=").unwrap();
        random_prefix.append(&mut plaintext);
        plaintext = pkcs7pad(&random_prefix, 16);

        aes_128_ecb_encode(&plaintext, &key).unwrap()
    }

    let mut prefix = Vec::new();
    let mut current_len: usize = enc_oracle(&prefix).len();
    while enc_oracle(&prefix).len() == current_len {
        prefix.push(b'A');
    }
    current_len = enc_oracle(&prefix).len();
    let mut block_size = 0;
    while enc_oracle(&prefix).len() == current_len {
        prefix.push(b'A');
        block_size += 1;
    }

    println!("Block size: {:?}", block_size);

    prefix = vec![b'A'; 3 * block_size];

    let ciphertext = enc_oracle(&prefix);

    let mut using_ecb = false;
    let mut prefix_blocks = 0;
    for b in 0..(ciphertext.len() / block_size - 1) {
        let mut blocks_same = true;
        for i in 0..block_size {
            blocks_same &= ciphertext[b * block_size + i] == ciphertext[(b + 1) * block_size + i];
        }
        if blocks_same {
            using_ecb = true;
            prefix_blocks = b;
        }
    }

    println!("Using ECB mode: {:?}", using_ecb);

    prefix = vec![b'A'; block_size];
    let mut prefix_padding = block_size;
    let ciphertext = enc_oracle(&prefix);
    while let Some(_) = prefix.pop() {
        let test_ciphertext = enc_oracle(&prefix);
        let mut blocks_same = true;
        for i in 0..block_size {
            blocks_same &= ciphertext[(prefix_blocks - 1) * block_size + i] == test_ciphertext[(prefix_blocks - 1) * block_size + i];
        }
        if blocks_same {
            prefix_padding -= 1;
        } else {
            break;
        }
    }

    println!("Determined length of random prefix: {:?}", (prefix_blocks * block_size) - prefix_padding);

    let mut known_bytes = Vec::new();

    for _ in 0..enc_oracle(&Vec::new()).len() {
        prefix = vec![b'A'; prefix_padding];
        let padding_length = block_size - (known_bytes.len() % block_size) - 1;
        for _ in 0..padding_length {
            prefix.push(b'A');
        }
        let challenge = enc_oracle(&prefix);
        prefix.append(&mut known_bytes.clone());
        let current_block = prefix_blocks + known_bytes.len() / block_size;
        for b in 0..u8::MAX {
            prefix.push(b);
            let candidate = enc_oracle(&prefix);
            let mut found_byte = true;
            for i in 0..block_size {
                found_byte &= challenge[block_size * current_block + i] == candidate[block_size * current_block + i];
            }
            if found_byte {
                known_bytes.push(b);
                break;
            }
            prefix.pop();
        }
    }

    println!("{:?}", to_ascii(&known_bytes));
}

fn challenge_15() {
    let test = pkcs7pad(&from_ascii("YELLOW SUBMARINE"), 16);
    let test2 = pkcs7pad(&from_ascii("ICE ICE BABY"), 16);
    let test3 = from_ascii("ICE ICE BABY\x05\x05\x05\x05");

    println!("{:?} => {:?}", test, pkcs7unpad(&test));
    println!("{:?} => {:?}", test2, pkcs7unpad(&test2));
    println!("{:?} => {:?}", test3, pkcs7unpad(&test3));
}

fn challenge_16() {
    fn enc_oracle(input: &Vec<u8>) -> Vec<u8> {
        let mut plaintext = from_ascii("comment1=cooking%20MCs;userdata=");
        plaintext.append(&mut input.clone());
        plaintext.append(&mut from_ascii(";comment2=%20like%20a%20pound%20of%20bacon"));

        //One-time randomly generated key
        let key  = from_base64("QiB1YBiIylHbtl477czO7w==").unwrap();

        plaintext = pkcs7pad(&plaintext, 16);

        aes_128_cbc_encode(&plaintext, &key, &vec![0u8; 16]).unwrap()
    }

    fn is_admin(input: &Vec<u8>) -> bool {
        let key  = from_base64("QiB1YBiIylHbtl477czO7w==").unwrap();

        let plaintext = aes_128_cbc_decode(&input, &key, &vec![0u8;16]).unwrap();

        to_ascii(&plaintext).contains(";admin=true;")
    }

    let mut test_data = vec![b'A'; 16];
    test_data.append(&mut from_ascii(":admin<true"));

    let mut ciphertext = enc_oracle(&test_data);
    ciphertext[32] ^= 0b1;
    ciphertext[38] ^= 0b1;

    println!("Admin: {:?}", is_admin(&ciphertext));
}

fn challenge_17() {
    fn enc_oracle() -> (Vec<u8>, Vec<u8>) {
        const PLAINTEXTS: [&str; 10] = [
            "MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
            "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
            "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
            "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
            "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
            "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
            "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
            "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
            "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
            "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93"
        ];

        let mut rng = rand::rng();
        let mut plaintext = from_base64(PLAINTEXTS[rng.random_range(0..10)]).unwrap();
        //let mut plaintext = from_ascii("AAAAAAAAAAAAAAA");

        //One-time randomly generated key
        let key  = from_base64("QiB1YBiIylHbtl477czO7w==").unwrap();

        let iv  = rand_bytes(16);

        plaintext = pkcs7pad(&plaintext, 16);

        //println!("{:?}", plaintext);

        (aes_128_cbc_encode(&plaintext, &key, &iv).unwrap(), iv)
    }

    fn is_padded(input: &Vec<u8>, iv: &Vec<u8>) -> bool {
        let key  = from_base64("QiB1YBiIylHbtl477czO7w==").unwrap();

        let plaintext = aes_128_cbc_decode(&input, &key, &iv).unwrap();

        //println!("{:?}", plaintext);

        pkcs7unpad(&plaintext).is_some()
    }

    let (mut ciphertext, mut orig_iv) = enc_oracle();
    let mut known_bytes = Vec::new();

    while ciphertext.len() > 0 {
        let remainder = ciphertext.split_off(16);
        let mut known_block: Vec<u8> = Vec::new();
        for i in 0..16 {
            let mut iv = orig_iv.clone();
            for j in 0..i {
                iv[16 - i + j] ^= known_block[j] ^ (i as u8 + 1);
            }
            for test_byte in 0..u8::MAX {
                if i == 0 && test_byte == 0 {
                    // Block might already be correctly padded, skip 0 byte to ignore original padding
                    // This will break if the original was correctly padded with 0x1, which we compensate for later
                    continue;
                }
                iv[15 - i] ^= test_byte;
                if is_padded(&ciphertext, &iv) {
                    known_block.insert(0, test_byte ^ (1 + i as u8));
                    //println!("{:?}", known_block);
                    break;
                }
                iv[15 - i] ^= test_byte;
            }
            if i == 0 && known_block.len() == 0 {
                // No byte produced correct padding, because the correct one was the one we ignored earlier
                known_block.push(1);
            }
        }
        //println!("{:?}", known_block);
        orig_iv = ciphertext;
        ciphertext = remainder;
        known_bytes.append(&mut known_block);
    }
    println!("{:?}", to_base64(&known_bytes));
    println!("{:?}", to_ascii(&known_bytes));

}

fn challenge_18() {
    let key = from_ascii("YELLOW SUBMARINE");
    let ciphertext = from_base64("L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==").unwrap();
    let plaintext = aes_128_ctr_decode(&ciphertext, &key, &vec![0;8]).unwrap();

    println!("{:?}", to_ascii(&plaintext));
}

fn challenge_19() {
    let key  = from_base64("QiB1YBiIylHbtl477czO7w==").unwrap();
    let mut file = File::open("./19.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let mut ciphertexts = Vec::new();
    for line in contents.lines() {
        ciphertexts.push(aes_128_ctr_encode(&from_base64(&line).unwrap(), &key, &vec![0; 8]).unwrap());
    }

    // See if any of the strings have common phrases at certain positions
    // Using 'And' found many correct looking sentence starts
    // 'Eig' shows up, trying 'Eight'
    // 'I hav' -> 'I have'
    // 'A terr' -> 'A terrible'
    // 'Being cert' -> 'Being certain'
    // 'Eighteenth-ce' -> 'Eighteenth-century'
    // 'Her nights in argu' -> 'Her nights in argument'
    // 'Eighteenth-century hou' -> 'Eighteenth-century houses'
    // 'Or polite meaningless wor' -> 'Or polite meaningless words'
    // Google: Poem is 'Easter' by WB Yeats, inputting longest sentence gives all ciphertexts decrypted
    let mut keystream_candidates = Vec::new();
    let found_string = from_ascii("He, too, has been changed in his turn,");

    for ct in &ciphertexts {
        let mut c = ct.clone();
        if c.len() > found_string.len() {
            c.split_off(found_string.len());
        }
        keystream_candidates.push(xor(&c, &found_string));
    }

    for cand in keystream_candidates {
        let mut prefixes = Vec::new();
        for ct in &ciphertexts {
            let mut pfx = ct.clone();
            let mut ccand = cand.clone();
            if pfx.len() > ccand.len() {
                pfx.split_off(ccand.len());
            } else {
                ccand.split_off(pfx.len());
            }
            pfx = xor(&pfx, &ccand);
            let mut skip = false;
            for i in 0..pfx.len() {
                if pfx[i].is_ascii_control() || "~<>=|#{}\\[]".contains(pfx[i] as char) {
                    skip = true;
                    break;
                }
            }
            if !skip {
                prefixes.push(to_ascii(&pfx));
            }
        }
        if prefixes.len() == ciphertexts.len() {
            println!("{}", prefixes.join("\n"));
        }
    }
}

fn challenge_20() {
    let key  = from_base64("QiB1YBiIylHbtl477czO7w==").unwrap();
    let mut file = File::open("./20.txt").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let mut ciphertexts = Vec::new();
    for line in contents.lines() {
        ciphertexts.push(aes_128_ctr_encode(&from_base64(&line).unwrap(), &key, &vec![0; 8]).unwrap());
    }

    let mut keysize = usize::MAX;
    for ct in &ciphertexts {
        if ct.len() < keysize {
            keysize = ct.len();
        }
    }

    for ct in &mut ciphertexts {
        ct.truncate(keysize);
    }

    let mut blocks = Vec::new();

    for j in 0..keysize {
        let mut block: Vec<u8> = Vec::new();
        for ct in &ciphertexts {
            block.push(ct[j]);
        }
        blocks.push(block);
    }

    let mut guessed_key = Vec::new();
    for block in blocks {
        let mut best_key: u8 = 0;
        let mut best_score = f64::MAX;
        for i in 0..=u8::MAX {
            let output = single_char_xor(&block, &i);
            let output_s = to_ascii(&output);
            let score = char_freq(&output_s);
            if score < best_score {
                best_score = score;
                best_key = i;
            }
        }
        guessed_key.push(best_key);
    }

    // for ct in &ciphertexts {
    //     let output = repeat_key_xor(&ct, &guessed_key);

    //     println!("{}", to_ascii(&output));
    // }
    println!("{:?}", to_hex(&guessed_key));
    // "be3c05cd45aff87036475e0ce9f300cb72634f2fddd82c54bf61768a63e89d6d32eeb87f5f80244ce54154803d0bec9e396aefb16d"
    // few letters are off, manually correcting
    let manual_key = from_hex("993c05cd45aff87036475e0ce9f300cb72634f2fddd82c54bf61768a62e89d6d32eeb87f5f80244ce54154803d0bec9e396aefb16d").unwrap();
    for ct in &ciphertexts {
        let output = repeat_key_xor(&ct, &manual_key);

        println!("{}", to_ascii(&output));
    }
}