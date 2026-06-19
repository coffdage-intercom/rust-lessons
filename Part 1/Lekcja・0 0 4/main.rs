// Wylistowanie fruits oraz jedno po drugim osobno:

fn main() {
    let fruits: [&str; 3] = ["apple", "orange", "banana"];
    println!("All fruits list: {:?}", fruits);
    println!("Fruit 1: {}", fruits[0]);
    println!("Fruit 2: {}", fruits[1]);
    println!("Fruit 3: {}", fruits[2]);
}