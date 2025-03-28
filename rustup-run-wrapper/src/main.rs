use std::env;
use std::process::{Command, Stdio};

fn main() {
    let cwd = env::current_dir().unwrap();
    let canonical = cwd.canonicalize().unwrap();

    let cmd = Command::new("rustup")
        .args(&[
            "run",
            "stable",
            "cargo",
            "check",
            "--target",
            "x86_64-pc-windows-msvc",
        ])
        .stdout(Stdio::piped())
        .current_dir(canonical);

    dbg!(&cmd);

    cmd.spawn().unwrap();
}
