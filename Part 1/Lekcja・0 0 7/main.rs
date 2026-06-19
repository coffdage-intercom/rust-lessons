use std::io::{self, Write};
use std::process::Command;

fn main() {

  loop { // loop beginning
    println!("\n=== MENU GŁÓWNE ===");
    println!("'hi'    =>   Powiedz cześć");
    println!("'ls'    =>   Wywołaj komendę systemową (ls / dir)");
    println!("'exit'  =>   Wyjdź z programu");
    print!("input >> ");

    io::stdout()
        .flush()
        .unwrap();

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Err! ");

    let input = input.trim();

    match input {
        "hi" => {
            println!("cześć");
            continue;
        }
        "ls" => {
            let status = Command::new("ls")
                .status();

            if status.is_err() {
                println!("Unknown command or Section err(!). Please read documentation 'Vp-Prog(menu-test)-G6J82F' for more informations.");
            }
        }
        "exit" => {
            break;
        }
        _ => {
            println!("Unknown command: ''' {} ''' err!.", input);
        }
    }

  } // loop end
}