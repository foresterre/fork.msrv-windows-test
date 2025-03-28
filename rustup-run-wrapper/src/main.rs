use std::process::{Command, Stdio};

fn main() {
    let _ = Command::new("rustup")
        .args(&["run", "stable", "cargo", "check"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
}
