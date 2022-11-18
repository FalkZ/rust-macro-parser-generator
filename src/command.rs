use std::process::Command;

fn run_command(command: &str) {
    let out = if cfg!(target_os = "windows") {
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

        if out.stderr.len() > 0 {println!("command failed: {} {:?}", &command ,&out)};

       
}

pub fn prettier_format(file: &str) {
    run_command(&format!("prettier --print-width=50 --write {}", file))
}

pub fn esbuild(file: &str) {
    run_command(&format!("pnpm exec esbuild {} --sourcemap --outfile={}", file, file.replace(".ts", ".js")))
}
