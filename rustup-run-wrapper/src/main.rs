use std::env;
use std::os::linux::raw::stat;
use std::process::{Command, ExitCode, Stdio, Termination};

fn main() -> impl Termination {
    let cwd = env::current_dir().unwrap();
    let canonical = cwd.canonicalize().unwrap();

    let mut cmd = Command::new("rustup");
    let cmd = cmd
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

    let child = cmd.spawn().unwrap();
    let output = child.wait_with_output().unwrap();
    let text = String::from_utf8_lossy(&output.stdout).into_owned();

    eprintln!(">>> {text}");

    let status = output.status;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{}", status.code().unwrap()))
    }
}
