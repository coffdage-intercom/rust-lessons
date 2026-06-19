// Program który korzysta z innego pliku *.rs w tym samym katalogu:

use std::io::{self, Write};
mod external;

fn main_self_func() {
    println!("Plik main.rs działa prawidłowo!.");
    println!("Plik main.rs funkcja 'main_self_func()' działa prawidłowo!.");
}

fn main() {
    println!("Wprowadzenie do programu i jego opcji.");
    println!("Dostępne opcje:");
    println!("'self' == Wywołanie funkcji 'main_self_func' z pliku main.rs");
    println!("'mod'  == Wywołanie funckji 'file_func' z pliku external.rs");

    print!("input >> ");

    io::stdout()
        .flush()
        .unwrap();

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Bład programu!");

    let input = input.trim();

    let _wynik = match input {
        "self" => {
            main_self_func();
        }
        "mod" => {
            external::file_func();
        }
        _ => {
            return;
        }
    };
}

// Wynik: [ Example == $ // Output == #output# ]

// Wprowadzenie do programu i jego opcji.
// Dostępne opcje:
// 'self' == Wywołanie funkcji 'main_self_func' z pliku main.rs
// 'mod'  == Wywołanie funckji 'file_func' z pliku external.rs
// input >> $input$
// #output#