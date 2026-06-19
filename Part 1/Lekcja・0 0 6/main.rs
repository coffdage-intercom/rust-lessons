use std::io::{self, Write}; // Potrzebne do pobierania danych i czyszczenia bufora terminala
use std::process::Command; // Potrzebne, jeśli chcesz wywołać komendę systemową

fn main() {
    // To jest nieskończona pętla (działa, dopóki użytkownik nie wybierze wyjścia)
    loop {
        println!("\n=== MENU GŁÓWNE ===");
        println!("1. Powiedz cześć");
        println!("2. Wywołaj komendę systemową (ls / dir)");
        println!("3. Wyjdź z programu");
        print!("Wybierz opcję (1-3): ");
        
        // Wymuszamy wyświetlenie tekstu przed pobraniem inputu
        io::stdout().flush().unwrap();

        // Tworzymy pusty tekst, do którego zapiszemy to, co wpiszesz
        let mut input = String::new();
        
        // Pobieramy linijkę z klawiatury
        io::stdin()
            .read_line(&mut input)
            .expect("Nie udało się odczytać linii");

        // Czyszczenie inputu: usuwamy białe znaki (np. enter na końcu)
        let input = input.trim();

        // Instrukcja MATCH - odpowiednik switch/case z innych języków
        match input {
            "1" => {
                println!("-> Siemanko! Rust jednak działa!");
            }
            "2" => {
                println!("-> Odpalam komendę systemową...");
                // Wywołanie komendy systemowej (dla Windowsa zmień "ls" na "dir")
                let status = Command::new("ls") 
                    .status();
                
                if status.is_err() {
                    println!("(Jeśli jesteś na Windowsie, zmień 'ls' w kodzie na 'dir')");
                }
            }
            "3" => {
                println!("-> Zamykam program. Nara!");
                break; // Przerywa pętlę 'loop', czyli kończy program
            }
            _ => {
                // To się wykona, jeśli wpiszesz cokolwiek innego niż 1, 2 lub 3
                println!("-> Ej, miała być opcja od 1 do 3. Spróbuj jeszcze raz.");
            }
        }
    }
}