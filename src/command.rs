use std::process::Command;

fn run_command(command: &str) {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .output()
            .expect(&format!("failed to execute: {}", command))
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect(&format!("failed to execute: {}", command))
    };
}

pub fn prettier_format(file: &str) {
    run_command(&format!("prettier --write {}", file))
}
