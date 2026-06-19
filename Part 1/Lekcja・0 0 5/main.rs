// Kalkulacja długości string'a:
use std::io::{self, Write};

fn main() {
    print!("Input your word/s: ");

    io::stdout()
        .flush()
        .unwrap();

    let mut s1 = String::new();

    io::stdin()
        .read_line(&mut s1)
        .expect("Err!");

    let len = lenght(&s1);
    let s2 = &s1.trim();

    println!("Your input: {}", s2);
    println!("Length of '{}' is {} characters.", s2, len);
}

fn lenght(s: &Str) -> usize {
    s.trim().len()
}
