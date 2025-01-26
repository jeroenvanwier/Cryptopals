mod bytestring;
mod text_analysis;
mod crypto;
use std::{fs::File, i32, io::Read, process::Output, result};

use bytestring::*;
use text_analysis::*;
use crypto::*;

fn main() {
    challenge_8();
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
        let mut remainder = line.clone().to_string();
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