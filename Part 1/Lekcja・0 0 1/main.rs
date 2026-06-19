// Program wyświetlający text:

fn main() {
    println!("println!() printuje text formatowany i z enterem automatycznie.");
    eprintln!("eprintln!() printuje text formatowany i z enterem automatycznie. Nakłada się nad print().");
    eprint!("eprint!() printuje text prosty nie formatowany, bez entera automatycznego. Nakłada się nad print!() ale jest pod eprintln!().");
    println!(" ");
    print!("print!() printuje text prosty nie formatowany, bez entera automatycznego. Pojawia się pod eprint!().");
    println!(" ");
}

// Wynik programu:

// println!() printuje text formatowany i z enterem automatycznie.
// eprintln!() printuje text formatowany i z enterem automatycznie. Nakłada się nad print().
// eprint!() printuje text prosty nie formatowany, bez entera automatycznego. Nakłada się nad print!() ale jest pod eprintln!().
// print!() printuje text prosty nie formatowany, bez entera automatycznego. Pojawia się pod eprint!().