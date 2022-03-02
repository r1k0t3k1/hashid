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
        println!("hash!");
    }
}
