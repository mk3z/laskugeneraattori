use std::process::Command;

fn main() {
    let commit_hash = std::env::var("GIT_COMMIT_SHA").unwrap_or_else(|_| {
        if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
            String::from_utf8(output.stdout).unwrap()
        } else {
            String::from("dirty")
        }
    });
    println!("cargo:rustc-env=COMMIT_HASH={}", commit_hash);
}
