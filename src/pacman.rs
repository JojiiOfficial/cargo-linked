use std::process::Command;

/// Return the owner of a given file
pub fn get_file_owner<S: AsRef<str>>(file: S) -> Option<String> {
    if let Ok(package) = Command::new("pacman")
        .args(&["-Qqo", file.as_ref()])
        .output()
    {
        return Some(String::from_utf8(package.stdout).unwrap().trim().to_owned());
    }

    None
}
