use super::*;

#[test]
fn as_manifest_path_arg_1() {
    let manifest_path = as_manifest_path_arg("/home/lol/programming/rust/AURtomatic/Cargo.toml");

    assert_eq!(
        manifest_path,
        "--manifest-path=/home/lol/programming/rust/AURtomatic/Cargo.toml"
    );
}

#[test]
fn as_manifest_path_arg_2() {
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

#[test]
fn test_to_manifest_file_1() {
    let inp = "/home/lol/programming/rust/AURtomatic/";

    assert_eq!(
        to_manifest_file(&inp),
        "/home/lol/programming/rust/AURtomatic/Cargo.toml"
    )
}

#[test]
fn test_to_manifest_file_2() {
    assert_eq!(to_manifest_file("."), "Cargo.toml")
}

#[test]
fn test_to_manifest_path() {
    let inp = "/home/lol/programming/rust/AURtomatic/Cargo.toml";

    assert_eq!(
        to_manifest_path(&inp),
        "/home/lol/programming/rust/AURtomatic"
    )
}

#[test]
fn test_get_build_folder() {
    assert_eq!(get_binary("."), "./target/debug/cargo-linked");
}
