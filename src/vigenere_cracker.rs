// src/vigenere_cracker.rs

use crate::vigenere::vigenere_decrypt;
use std::collections::HashMap;

// You should replace these with the actual n-gram frequencies.
// Standard English letter frequencies
const ENGLISH_SINGLE_FREQUENCIES: &str = "\
A 0.08167
B 0.01492
C 0.02782
D 0.04253
E 0.12702
F 0.02228
G 0.02015
H 0.06094
I 0.06966
J 0.00153
K 0.00772
L 0.04025
M 0.02406
N 0.06749
O 0.07507
P 0.01929
Q 0.00095
R 0.05987
S 0.06327
T 0.09056
U 0.02758
V 0.00978
W 0.02360
X 0.00150
Y 0.01974
Z 0.00074";

const ENGLISH_BIGRAM_FREQUENCIES: &str = "\
TH 0.0387
HE 0.0367
IN 0.0225
ER 0.0210
AN 0.0205
RE 0.0180
ES 0.0175
ON 0.0170
ST 0.0165
NT 0.0160";

const ENGLISH_TRIGRAM_FREQUENCIES: &str = "\
THE 0.0210
AND 0.0100
ING 0.0075
HER 0.0060
THA 0.0055
ERE 0.0050
ENT 0.0045
ION 0.0040
EDT 0.0035
EST 0.0030";

const ENGLISH_QUADGRAM_FREQUENCIES: &str = "\
THAT 0.0020
THER 0.0015
WITH 0.0010
TION 0.0009
HERE 0.0008
ANCE 0.0007
MENT 0.0006
IGHT 0.0005
HAVE 0.0004";

// This function computes the correlation score between two n-gram frequency distributions.
fn correlation_score(freq1: &HashMap<String, f32>, freq2: &HashMap<String, f32>) -> f32 {
    freq1
        .iter()
        .map(|(ngram, freq)| freq * freq2.get(ngram).unwrap_or(&0.0))
        .sum()
}

// This function returns the n-gram frequency distribution of a given text.
fn ngram_frequency_distribution(text: &str, n: usize) -> HashMap<String, f32> {
    let mut ngram_freqs = HashMap::new();
    let mut count = 0;

    for chars in text.chars().collect::<Vec<_>>().windows(n) {
        let ngram = chars.iter().collect::<String>().to_ascii_uppercase();
        *ngram_freqs.entry(ngram).or_insert(0.0) += 1.0;
        count += 1;
    }

    // Normalize the frequencies
    ngram_freqs
        .iter_mut()
        .for_each(|(_, freq)| *freq /= count as f32);
    ngram_freqs
}

// This function pretty prints the n-gram frequency distribution
fn pretty_print_ngram_frequency_distribution(ngram_freqs: &HashMap<String, f32>) {
    // Sort the n-grams by frequency
    let mut sorted_ngrams: Vec<(&String, &f32)> = ngram_freqs.iter().collect();
    sorted_ngrams.sort_unstable_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    // Print the n-gram frequency distribution
    println!("N-gram | Frequency");
    println!("-------------------");
    for (ngram, frequency) in sorted_ngrams {
        println!("{:<6} | {:>10.4}", ngram, frequency);
    }
}

//fn parse_english_ngram_frequencies(n: usize, ngram_frequencies: &str) -> HashMap<String, f32> {
//    let mut ngram_freqs = HashMap::new();
//    if n == 1 {
//        for item in ngram_frequencies.split(',') {
//            let parts: Vec<&str> = item.split(':').collect();
//            let (ngram, freq) = (parts[0].to_string(), parts[1].parse::<f32>().unwrap());
//            ngram_freqs.insert(ngram, freq);
//        }
//    } else {
//        for (index, ngram) in ngram_frequencies.split(',').enumerate() {
//            ngram_freqs.insert(ngram.to_string(), index as f32);
//        }
//    }
//    ngram_freqs
//}
fn parse_english_ngram_frequencies(n: usize, english_freqs_raw: &str) -> HashMap<String, f32> {
    english_freqs_raw
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let ngram = parts.next().unwrap().to_ascii_uppercase();
            let freq = parts.next().unwrap().parse::<f32>().unwrap();
            (ngram, freq)
        })
        .collect()
}

pub fn text_score(text: &str, n: usize) -> f32 {
    let english_ngram_frequencies = match n {
        1 => parse_english_ngram_frequencies(n, ENGLISH_SINGLE_FREQUENCIES),
        2 => parse_english_ngram_frequencies(n, ENGLISH_BIGRAM_FREQUENCIES),
        3 => parse_english_ngram_frequencies(n, ENGLISH_TRIGRAM_FREQUENCIES),
        4 => parse_english_ngram_frequencies(n, ENGLISH_QUADGRAM_FREQUENCIES),
        _ => panic!("Unsupported n-gram length"),
    };
    let text_ngram_freqs = ngram_frequency_distribution(text, n);
    correlation_score(&text_ngram_freqs, &english_ngram_frequencies)
}

pub fn frequency_analysis_top_three(
    group: &str,
    n: usize,
    english_freqs: &HashMap<String, f32>,
) -> Vec<(f32, String)> {
    let group_freqs = ngram_frequency_distribution(group, n);
    pretty_print_ngram_frequency_distribution(&group_freqs);
    let group_freqs_vec: Vec<(String, f32)> =
        group_freqs.iter().map(|(k, v)| (k.clone(), *v)).collect();

    let mut scores: Vec<(f32, String)> = Vec::new();

    for (ngram, &freq) in english_freqs {
        let shifted_ngrams: Vec<(&String, f32)> = group_freqs_vec
            .iter()
            .enumerate()
            .map(|(i, (k, v))| (&group_freqs_vec[(i + n) % group_freqs_vec.len()].0, *v))
            .collect();

        let shifted_freqs: HashMap<String, f32> = shifted_ngrams
            .into_iter()
            .map(|(k, v)| (k.clone(), v))
            .collect();

        let score = correlation_score(&shifted_freqs, english_freqs);

        scores.push((score, ngram.to_string()));
    }

    scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    scores.into_iter().take(3).collect()
}

// Helper function to shift a string by a given key
fn shift_string_by(s: &str, key: &str) -> String {
    s.chars()
        .zip(key.chars().cycle())
        .map(|(c1, c2)| {
            let new_char = (((c1 as u8 - b'A') + (c2 as u8 - b'A')) % 26) as u8 + b'A';
            new_char as char
        })
        .collect()
}

fn shift_ngram(ngram: &str, n: usize, shift: usize) -> String {
    ngram
        .chars()
        .map(|c| {
            let offset = (c as u8 - b'A' + (shift % 26) as u8) % 26;
            (b'A' + offset) as char
        })
        .collect()
}

fn number_to_key(n: usize, num: usize) -> String {
    let mut key = String::new();
    let mut number = num;
    for _ in 0..n {
        key.push(((number % 26) as u8 + b'A') as char);
        number /= 26;
    }
    key
}

pub fn break_vigenere_with_known_key_length(
    ciphertext: &str,
    key_length: usize,
    n: usize,
    top_n: usize,
) -> Vec<(String, String, f32)> {
    let arr = match n {
        1 => ENGLISH_SINGLE_FREQUENCIES,
        2 => ENGLISH_BIGRAM_FREQUENCIES,
        3 => ENGLISH_TRIGRAM_FREQUENCIES,
        4 => ENGLISH_QUADGRAM_FREQUENCIES,
        _ => panic!("Unsupported n-gram length"),
    };
    let english_freqs = create_freq_map(arr);
    let mut groups: Vec<String> = vec![String::new(); key_length];
    let mut top_keys: Vec<Vec<String>> = vec![vec![String::new(); 3]; key_length];

    for (i, ch) in ciphertext.chars().enumerate() {
        groups[i % key_length].push(ch);
    }

    for (i, group) in groups.iter().enumerate() {
        let top_3_keys = frequency_analysis_top_three(&group, n, &english_freqs);
        top_keys[i] = top_3_keys.clone().into_iter().map(|(_, key)| key).collect();
        for (count, k) in top_3_keys.into_iter().enumerate() {
            println!("Key char {} has {:?} as #{}", i, k, count);
        }
    }

    let key_combinations = generate_key_combinations(&top_keys[..]);

    println!("Made it here{}", key_combinations.len());

    let mut scored_texts: Vec<(String, String, f32)> = key_combinations
        .into_iter()
        .map(|key| {
            let decrypted_text = vigenere_decrypt(ciphertext, &key);
            let score = text_score(&decrypted_text, n);
            (key, decrypted_text, score)
        })
        .collect();

    scored_texts
        .sort_unstable_by(|(_, _, score1), (_, _, score2)| score2.partial_cmp(score1).unwrap());

    println!("{}", scored_texts.len());
    scored_texts.into_iter().take(top_n).collect()
}
// Generate all possible combinations of the top 3 key strings for each key character.
fn generate_key_combinations(keys: &[Vec<String>]) -> Vec<String> {
    let mut combinations = vec![];

    // A helper function to generate key combinations recursively.
    fn generate_combinations_recursively(
        keys: &[Vec<String>],             // The key strings for each position.
        current_combination: &mut String, // The current key combination being built.
        index: usize,                     // The index of the current key string.
        combinations: &mut Vec<String>,   // The container to store the generated combinations.
    ) {
        // If the index reaches the end of the key strings array, add the current combination to the container.
        if index == keys.len() {
            combinations.push(current_combination.clone());
        } else {
            // Iterate through the key strings at the current index.
            for key_string in &keys[index] {
                // Add the current key string to the current combination.
                current_combination.push_str(key_string);
                // Recursively generate combinations with the updated current combination and the next index.
                generate_combinations_recursively(
                    keys,
                    current_combination,
                    index + 1,
                    combinations,
                );
                // Remove the last n characters from the current combination to backtrack and try other combinations.
                let n = key_string.len();
                current_combination.truncate(current_combination.len() - n);
            }
        }
    }

    // Call the helper function with an empty string as the initial current combination and index 0.
    generate_combinations_recursively(keys, &mut String::new(), 0, &mut combinations);
    combinations
}

fn create_freq_map(freq_str: &str) -> HashMap<String, f32> {
    let mut freq_map: HashMap<String, f32> = HashMap::new();

    for entry in freq_str.split(',') {
        let key_value: Vec<&str> = entry.split(':').collect();
        if key_value.len() == 2 {
            let key = key_value[0].to_string();
            let value = key_value[1].parse::<f32>().unwrap_or(0.0);
            freq_map.insert(key, value);
        }
    }

    freq_map
}

// ... rest of the code remains the same ...
// src/vigenere_cracker.rs

//use crate::vigenere::vigenere_decrypt;
//
//// Standard English letter frequencies
//const ENGLISH_FREQUENCIES: [f32; 26] = [
//    0.082, 0.015, 0.028, 0.043, 0.127, 0.022, 0.020, 0.061, 0.070, 0.002, 0.008, 0.040, 0.024,
//    0.067, 0.075, 0.019, 0.001, 0.060, 0.063, 0.091, 0.028, 0.010, 0.023, 0.001, 0.020, 0.001,
//];
//// This function computes the correlation score between two frequency distributions.
//fn correlation_score(freq1: &[f32], freq2: &[f32]) -> f32 {
//    freq1.iter().zip(freq2).map(|(a, b)| a * b).sum()
//}
//
//// This function returns the frequency distribution of a given text.
//fn frequency_distribution(text: &str) -> Vec<f32> {
//    let mut freqs = vec![0f32; 26];
//    let mut count = 0;
//
//    for ch in text.chars() {
//        if ch.is_ascii_alphabetic() {
//            let index = ch.to_ascii_uppercase() as usize - 'A' as usize;
//            freqs[index] += 1.0;
//            count += 1;
//        }
//    }
//
//    // Normalize the frequencies
//    freqs.iter_mut().for_each(|freq| *freq /= count as f32);
//    freqs
//}
//pub fn text_score(text: &str) -> f32 {
//    let text_freqs = frequency_distribution(text);
//    correlation_score(&text_freqs, &ENGLISH_FREQUENCIES)
//}
//
//
