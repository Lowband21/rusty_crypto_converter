// tests/crypto_tests.rs

use std::process::Command;

fn run_with_args(args: &[&str]) -> String {
    let output = Command::new("cargo")
        .arg("run")
        .args(args)
        .output()
        .expect("Failed to run the program with provided arguments.");
    String::from_utf8(output.stdout).expect("Output is not valid UTF-8.")
}

#[test]
fn test_caesar_cipher() {
    let encrypted = run_with_args(&["ceasar_e", "Hello, World!", "3"]);
    assert_eq!(encrypted.trim(), "khoorzruog");

    let decrypted = run_with_args(&["ceasar_d", "khoorzruog", "3"]);
    assert_eq!(decrypted.trim(), "helloworld");
}

#[test]
fn test_vigenere_cipher() {
    let encrypted = run_with_args(&["vigenere_e", "Hello, World!", "crypto"]);
    assert_eq!(encrypted.trim(), "jvjahkqijs");

    let decrypted = run_with_args(&["vigenere_d", "jvjahkqijs", "crypto"]);
    assert_eq!(decrypted.trim(), "helloworld");
}

#[test]
fn test_multiplication_table_mod_n() {
    let output = run_with_args(&["mult_table", "5"]);
    let expected_output = "\
        Multiplication Table (mod 5)
   1   2   3   4   0
   2   4   1   3   0
   3   1   4   2   0
   4   3   2   1   0
   0   0   0   0   0
Multiplicative Inverses (mod 5):
1 * 1 ≡ 1 (mod 5)
2 * 3 ≡ 1 (mod 5)
3 * 2 ≡ 1 (mod 5)
4 * 4 ≡ 1 (mod 5)
Count: 4";
    assert_eq!(output.trim(), expected_output);
}
