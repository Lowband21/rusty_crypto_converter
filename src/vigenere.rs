// src/vigenere.rs
pub fn letter_to_number(c: char) -> Option<u8> {
    let c = c.to_ascii_lowercase();
    if c >= 'a' && c <= 'z' {
        Some(c as u8 - 'a' as u8)
    } else {
        None
    }
}

pub fn number_to_letter(n: u8) -> Option<char> {
    if n < 26 {
        Some((n + 'a' as u8) as char)
    } else {
        None
    }
}

pub fn string_to_numbers(s: &str) -> Vec<u8> {
    s.chars()
        .filter_map(letter_to_number)
        .collect()
}

pub fn numbers_to_string(numbers: &[u8]) -> String {
    numbers.iter()
        .filter_map(|&n| number_to_letter(n))
        .collect()
}

pub fn vigenere_encrypt(plaintext: &str, key: &str) -> String {
    let plaintext_numbers = string_to_numbers(plaintext);
    let key_numbers = string_to_numbers(key);

    let encrypted_numbers = plaintext_numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| (n + key_numbers[i % key_numbers.len()]) % 26)
        .collect::<Vec<u8>>();

    numbers_to_string(&encrypted_numbers)
}

pub fn vigenere_decrypt(ciphertext: &str, key: &str) -> String {
    let ciphertext_numbers = string_to_numbers(ciphertext);
    let key_numbers = string_to_numbers(key);

    let decrypted_numbers = ciphertext_numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| (n + 26 - key_numbers[i % key_numbers.len()]) % 26)
        .collect::<Vec<u8>>();

    numbers_to_string(&decrypted_numbers)
}