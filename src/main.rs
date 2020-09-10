#![warn(dead_code)]

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut path = String::from(".");
    let mut args = env::args();
    if args.len() > 1 {
        path = args.nth(1).unwrap();
    }

    if !build(&path) {
        return;
    }
}

/// Build with cargo
fn build<S: AsRef<str>>(spath: S) -> bool {
    let manifest_val = as_manifest_path_arg(spath);
    let mut args: Vec<&str> = vec![&manifest_val];
    args = args.into_iter().filter(|&i| !i.is_empty()).collect();

    Command::new("cargo")
        .arg("build")
        .args(&mut args)
        .status()
        .is_ok()
}

/// Create cargo manifest_path parameter
fn as_manifest_path_arg<S: AsRef<str>>(spath: S) -> String {
    if spath.as_ref() == "." {
        return "".to_owned();
    }

    let path = spath.as_ref();
    let mut path = Path::new(path).to_path_buf();
    if !spath.as_ref().ends_with("Cargo.toml") {
        path = path.join("Cargo.toml");
    }

    format!("--manifest-path={}", &path.to_str().unwrap())
}

//fn to_manifest_file()

#[allow(dead_code)]
fn get_file_owner<S: AsRef<str>>(file: S) -> Option<String> {
    if let Ok(package) = Command::new("pacman")
        .args(&["-Qqo", file.as_ref()])
        .output()
    {
        return Some(String::from_utf8(package.stdout).unwrap().trim().to_owned());
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_manifest_path_arg_1() {
        let manifest_path = as_manifest_path_arg("/home/lol/programming/rust/AURtomatic");

        assert_eq!(
            manifest_path,
            "--manifest-path=/home/lol/programming/rust/AURtomatic/Cargo.toml"
        );
    }

    #[test]
    fn as_manifest_path_arg_2() {
        let manifest_path =
            as_manifest_path_arg("/home/lol/programming/rust/AURtomatic/Cargo.toml");

        assert_eq!(
            manifest_path,
            "--manifest-path=/home/lol/programming/rust/AURtomatic/Cargo.toml"
        );
    }

    #[test]
    fn as_manifest_path_arg_3() {
        assert!(as_manifest_path_arg(".").is_empty());
    }

    // Requires pacman and glibc
    #[test]
    fn test_get_file_owner() {
        let file = "/usr/lib/libc.so";

        let output = get_file_owner(file);
        assert!(output.is_some());
        assert_eq!(output.unwrap(), "glibc");
    }
}
