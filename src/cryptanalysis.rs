// src/analysis.rs
//

use std::collections::HashMap;

fn index_of_coincidence(text: &str) -> f64 {
    let mut frequency_map: HashMap<char, usize> = HashMap::new();

    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            *frequency_map.entry(ch.to_ascii_uppercase()).or_insert(0) += 1;
        }
    }

    let total_chars = text.chars().filter(|ch| ch.is_ascii_alphabetic()).count() as f64;
    let sum_of_products: f64 = frequency_map
        .iter()
        .map(|(_, &count)| count as f64 * (count as f64 - 1.0))
        .sum();

    sum_of_products / (total_chars * (total_chars - 1.0))
}

pub fn frequency_analysis(ciphertext: &str) {
    let text = ciphertext
        .chars()
        .filter(|&ch| ch.is_alphabetic() && ch != '\n')
        .collect::<String>();

    let total_chars = text.chars().count() as f64;
    let mut frequency_map: HashMap<char, usize> = HashMap::new();

    for ch in text.chars() {
        let count = frequency_map.entry(ch).or_insert(0);
        *count += 1;
    }

    let mut frequency_vec: Vec<(char, f64)> = frequency_map
        .iter()
        .map(|(&ch, &count)| (ch, count as f64 / total_chars))
        .collect();

    let highest_freq = frequency_vec
        .iter()
        .map(|(_, freq)| freq)
        .cloned()
        .fold(f64::MIN, f64::max);

    let most_common_letters: Vec<&char> = frequency_vec
        .iter()
        .filter(|(_, freq)| (freq - highest_freq).abs() < f64::EPSILON)
        .map(|(ch, _)| ch)
        .collect();

    let lowest_freq = frequency_vec
        .iter()
        .map(|(_, freq)| freq)
        .cloned()
        .fold(f64::MAX, f64::min);

    let least_common_letters: Vec<&char> = frequency_vec
        .iter()
        .filter(|(_, freq)| (freq - lowest_freq).abs() < f64::EPSILON)
        .map(|(ch, _)| ch)
        .collect();

    frequency_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let least_common1 = frequency_vec.get(0).unwrap();
    let least_common2 = frequency_vec.get(1).unwrap();
    let highest_freq = frequency_vec.last().unwrap().1;

    let most_common_letters: Vec<&char> = frequency_vec
        .iter()
        .filter(|(_, freq)| (freq - highest_freq).abs() < f64::EPSILON)
        .map(|(ch, _)| ch)
        .collect();

    println!(
        "For the ciphertext given below, the least common letters are {} and {} with a frequency of {:.3} and {:.3}.",
        least_common1.0, least_common2.0, least_common1.1, least_common2.1
    );

    println!(
        "There are {} letters tied for the most-frequent, with a frequency of {:.3}.",
        most_common_letters.len(),
        highest_freq
    );
    println!(
        "One of the most frequent letters is {}.",
        most_common_letters[0]
    );
    println!(
        "The index of coincidence is {:.3}",
        index_of_coincidence(ciphertext)
    )
}
use crate::ceasar::decrypt;

pub fn brute_force_caesar(ciphertext: &str) {
    println!("Brute Force Caesar Cipher Decryption:");

    for shift in 1..=26 {
        let decrypted_text = decrypt(ciphertext, shift);
        println!("Shift {}: {}", shift, decrypted_text);
    }
}
