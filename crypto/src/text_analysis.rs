pub const CHAR_FREQS: [f64; 26] = [0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074];

pub fn char_freq(input: &String) -> f64 {
    let mut counts = [0.0; 26];
    let mut total_letters = 0.0;
    let mut sum = 0.0;

    for c in input.chars() {
        let n = c.to_ascii_lowercase() as usize;
        if n >= 97 && n <= 122 {
            counts[n - 97] += 1.0;
            total_letters += 1.0;
        } else if !c.is_ascii() {
            sum += 0.1;
        }
        if c.is_uppercase() {
            sum += 0.01;
        }
    }
    for i in 0..26 {
        sum += ((counts[i] / total_letters) - CHAR_FREQS[i]).abs();
    }
    sum
}

pub fn strike_unprintable_characters(input: &String) -> String {
    let mut output = String::new();
    for c in input.chars() {
        if !c.is_ascii_control() {
            output.push(c);
        } else {
            output.push('-');
        }
    }
    output
}