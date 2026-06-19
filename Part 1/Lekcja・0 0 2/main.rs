// Program wprowadzania danych:

use std::io;

fn main() {
    eprint!("Wprowadź liczbę przykładową: ");

    let mut example: String = String::new();
    
    io::stdin()
        .read_line(&mut example)
        .expect("Błąd");

    print!("Wprowadziłeś text/string: {}", example);
}

// Wynik: [ EXAMPLE = $ ]

// Wprowadź liczbę przykładową: $test$
// Wprowadziłeś text/string: $test$