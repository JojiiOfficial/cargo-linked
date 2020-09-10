use cargo_metadata::Metadata;
use toml_edit::Document;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::ldd::LDD;

pub struct App {
    metadata: Metadata,
}

// Public functions
impl App {
    pub fn new(metadata: Metadata) -> Self {
        App { metadata }
    }

    pub fn run(&self) {
        //self.build();
        let ldd = LDD::from_app(&self);
        let linked = ldd.linked_files().trim();
        for i in linked.items {
            println!("{}", i);
        }
    }

    pub fn get_binary(&self) -> String {
        let path = Path::new(&self.metadata.target_directory)
            .join("debug")
            .join(self.get_crate_name());

        String::from(path.to_str().unwrap())
    }
}

// Private functions
impl App {
    fn get_cargo_toml(&self) -> PathBuf {
        self.metadata.workspace_root.join("Cargo.toml")
    }

    fn get_crate_name(&self) -> String {
        self.get_package_info(&self.read_cargo_toml(), "name")
    }

    fn read_cargo_toml(&self) -> Document {
        let content = fs::read_to_string(self.get_cargo_toml()).expect("Can't read Cargo.toml");
        content.parse::<Document>().expect("Can't parse Cargo.toml")
    }

    fn get_package_info(&self, parsed: &Document, key: &str) -> String {
        String::from(parsed["package"][key].as_str().unwrap())
    }

    /// Create cargo manifest_path parameter
    fn as_manifest_path_arg(&self) -> String {
        format!(
            "--manifest-path={}",
            &self.get_cargo_toml().to_str().unwrap()
        )
    }

    /// Build with cargo
    fn build(&self) -> bool {
        let manifest_val = self.as_manifest_path_arg();
        let mut args: Vec<&str> = vec![&manifest_val];
        args = args.into_iter().filter(|&i| !i.is_empty()).collect();

        Command::new("cargo")
            .arg("build")
            .args(&mut args)
            .status()
            .is_ok()
    }
}
