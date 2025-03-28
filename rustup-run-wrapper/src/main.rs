use std::process::{Command, Stdio};

fn main() {
    let _ = Command::new("rustup")
        .args(&["run", "stable", "cargo", "check", "--target", "x86_64-pc-windows-msvc"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
}
