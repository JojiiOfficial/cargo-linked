use std::process::Command;

use crate::app::BinaryType;
use crate::pacman;
use crate::App;

pub struct LDD<'a> {
    app: &'a App,
}

#[derive(Default, Debug)]
pub struct LddResult {
    pub items: Vec<LddObject>,
}

#[derive(Default, Debug, Clone)]
pub struct LddObject {
    package: String,
    file: String,
}

impl std::fmt::Display for LddObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.package)
    }
}

// Public
impl<'a> LDD<'a> {
    pub fn from_app(app: &'a App) -> Self {
        LDD { app }
    }
}

// Private
impl<'a> LDD<'a> {
    /// Get linked files.
    pub fn linked_files(&self, bin_type: BinaryType) -> LddResult {
        // execute ldd commant to get linked files
        let output = Command::new("ldd")
            .arg(&self.app.get_binary(bin_type))
            .output();
        if output.is_err() {
            return LddResult::default();
        }
        let package = output.unwrap();

        LddResult {
            items: String::from_utf8(package.stdout)
                .unwrap()
                .lines()
                .into_iter()
                .filter(|f| !f.trim().to_owned().is_empty() && f.trim().to_owned().contains("=>"))
                .map(|m| m.to_owned())
                .map(|m| {
                    // Strip the file from the output data
                    let file = m.split(" ").nth(2).unwrap().to_owned();
                    let mut package = String::new();
                    if !file.is_empty() {
                        if let Some(owner) = pacman::get_file_owner(&file) {
                            package = owner;
                        }
                    }

                    LddObject { file, package }
                })
                .collect(),
        }
    }
}

impl LddResult {
    pub fn trim(&self) -> Self {
        let mut v: Vec<LddObject> = Vec::new();

        for i in self.items.iter() {
            if !Self::has_package(&v, &i.package) {
                v.push(i.clone());
            }
        }

        LddResult { items: v }
    }

    fn has_package(v: &Vec<LddObject>, package: &String) -> bool {
        for i in v {
            if i.package == *package {
                return true;
            }
        }

        false
    }
}
