// src/main.rs
use std::env;
use std::process;

use crypto_converter::{string_to_numbers, numbers_to_string};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <to_num|to_str> \"<input>\"", args[0]);
        process::exit(1);
    }

    let mode = &args[1];
    let input = &args[2];

    match mode.as_str() {
        "to_num" => {
            let numbers = string_to_numbers(input);
            let numbers_str = numbers
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", numbers_str);
        }
        "to_str" => {
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
        _ => {
            eprintln!("Invalid mode. Use 'to_num' or 'to_str'.");
            process::exit(1);
        }
    }
}
