#![warn(dead_code)]

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use toml_edit::Document;

#[cfg(test)]
#[path = "main_test.rs"]
mod main_tests;

fn main() {
    let mut path = String::from(".");
    let mut args = env::args();
    if args.len() > 1 {
        path = args.nth(1).unwrap();
    }

    if !build(&path) {
        return;
    }

    let bin_file = get_binary(&path);
    let binary = Path::new(&bin_file);
    if !binary.exists() {
        println!("Binary was not found");
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
    let path = to_manifest_file(&path);

    format!("--manifest-path={}", &path)
}

fn to_manifest_path<S: AsRef<str>>(path: S) -> String {
    let p = Path::new(path.as_ref());

    if path.as_ref().ends_with("Cargo.toml") {
        return String::from(p.parent().unwrap().to_str().unwrap());
    }

    String::from(p.to_str().unwrap())
}

fn to_manifest_file<S: AsRef<str>>(path: S) -> String {
    if path.as_ref() == "." {
        return "Cargo.toml".to_owned();
    }

    let mut p = Path::new(path.as_ref()).to_path_buf();

    if !path.as_ref().ends_with("Cargo.toml") {
        p = p.join("Cargo.toml");
    }

    String::from(p.to_str().unwrap())
}

fn get_file_owner<S: AsRef<str>>(file: S) -> Option<String> {
    if let Ok(package) = Command::new("pacman")
        .args(&["-Qqo", file.as_ref()])
        .output()
    {
        return Some(String::from_utf8(package.stdout).unwrap().trim().to_owned());
    }

    None
}

fn get_binary<S: AsRef<str>>(base: S) -> String {
    let manifest_path = to_manifest_path(&base);

    let pkg_name = get_package_info(&read_cargo_toml(to_manifest_file(&base)), "name");

    let path = Path::new(&manifest_path)
        .join("target")
        .join("debug")
        .join(pkg_name);

    String::from(path.to_str().unwrap())
}

fn read_cargo_toml<S: AsRef<str>>(path: S) -> Document {
    let content = fs::read_to_string(to_manifest_file(path)).expect("Can't read Cargo.toml");
    content.parse::<Document>().expect("Can't parse Cargo.toml")
}

fn get_package_info(parsed: &Document, key: &str) -> String {
    String::from(parsed["package"][key].as_str().unwrap())
}
