use std::io::{stdin, stdout, Write};

const WELCOME_MESSAGE: &str = "HASH ID";
const VERSION: &str = "1.0.0";

fn main() {
    println!("{}",WELCOME_MESSAGE);
    println!("{}",VERSION);

    let mut stdout = stdout();

    let stdin = stdin();
    let mut hash = String::new();
    loop {
        print!("Insert hash:");
        stdout.flush().unwrap();
        stdin.read_line(&mut hash).unwrap();
        hash = hash.trim().to_string();
        println!("{}",md5(&hash));
    }
}

fn is_digit_and_alphabet(s: &String) -> bool {
    let mut is_digit_appear = false;
    let mut is_alphabet_appear = false;

    for c in s.chars() {
        if !c.is_ascii_alphanumeric() { return false }
        is_digit_appear |= c.is_ascii_digit();
        is_alphabet_appear |= c.is_ascii_alphabetic();
    }
    is_digit_appear && is_alphabet_appear
}

#[test]
fn is_digit_and_alphabet_test() {
    let number = "0123456789".to_string();
    let alphabet = "abcdefghijklmn".to_string();
    let alnum = "012346789abcde".to_string();
    let hello = "5d41402abc4b2a76b9719d911017c592".to_string();
    let symbol = "^%$)(*!&2)#$".to_string();

    assert_eq!(is_digit_and_alphabet(&number),false);
    assert_eq!(is_digit_and_alphabet(&alphabet),false);
    assert_eq!(is_digit_and_alphabet(&alnum),true);
    assert_eq!(is_digit_and_alphabet(&hello),true);
    assert_eq!(is_digit_and_alphabet(&symbol),false);
}

fn md5(hash: &String) -> bool {
    let example_hash = "8743b52063cd84097a65d1633f5c74f5";
    if hash.len() == example_hash.len() && is_digit_and_alphabet(hash) { return true }
    false
}

