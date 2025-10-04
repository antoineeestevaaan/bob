use std::env;
use std::fs;
use std::process::Command;

const MAIN_BRANCH: &str = "master";
const VERSION_FILE: &str = "version.txt";

fn main() {
    let base_version = env::var("CARGO_PKG_VERSION").unwrap();

    let git_describe = Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty=-dirty"])
        .output();

    let version_string = match git_describe {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => base_version,
    };

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/{MAIN_BRANCH}");
    fs::write(VERSION_FILE, &version_string).unwrap();
}
