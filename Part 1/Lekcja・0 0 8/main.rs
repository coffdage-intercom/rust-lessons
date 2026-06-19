use std::process::Command;

fn main() {
    println!("Fixing problem: 'VT-x is being used by (...)'");

    Command::new("rmmod")
        .arg("kvm_intel")
        .status()
        .unwrap();

    Command::new("rmmod")
        .arg("kvm")
        .status()
        .unwrap();

    println!("Problem fixed! ^ - ^");
}

