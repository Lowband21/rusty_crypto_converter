// src/main.rs
use std::env;
use std::process;

mod ceasar;
use ceasar::{string_to_numbers, numbers_to_string};
mod vigenere;
use vigenere::{vigenere_encrypt, vigenere_decrypt};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <ceasar_e|ceasar_d|vigenere_e|vigenere_d> \"<input>\" \"<key>\"", args[0]);
        process::exit(1);
    }

    let mode = &args[1];
    let input = &args[2];
    let key = &args[3];

    match mode.as_str() {
        "ceasar_e" => {
            let numbers = string_to_numbers(input);
            let numbers_str = numbers
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", numbers_str);
        }
        "ceasar_d" => {
            let numbers_result = input
                .split_whitespace()
                .map(|s| s.parse::<u8>())
                .collect::<Result<Vec<u8>, _>>();

            match numbers_result {
                Ok(numbers) => {
                    let string = numbers_to_string(&numbers);
                    println!("{}", string);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        "vigenere_e" => {
            let encrypted = vigenere_encrypt(input, key);
            println!("{}", encrypted);
        }
        "vigenere_d" => {
            let decrypted = vigenere_decrypt(input, key);
            println!("{}", decrypted);
        }
        _ => {
            eprintln!("Invalid mode. Use 'ceasar_e', 'ceasar_d', 'vigenere_e', or 'vigenere_d'.");
            process::exit(1);
        }
    }
}
