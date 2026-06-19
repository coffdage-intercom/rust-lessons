use sts::process::Command;

fn main() {
    let commands = vec![
        ("echo", vec!["First line code of command."]),
        ("echo", vec!["First line code of command."]),
    ];

    for (cmd, args) in commands {
        Command::new(cmd)
            .args(&args)
            .status()
            .unwrap();
    }
}