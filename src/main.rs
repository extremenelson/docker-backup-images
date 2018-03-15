use std::process::Command;

fn main() {
    println!("Hello, world!");

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/c", "echo helo"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .args(&["/C", "echo hello"])
            .output()
            .expect("Failed to execute process")
    };

    let hello = output.stdout;
}
