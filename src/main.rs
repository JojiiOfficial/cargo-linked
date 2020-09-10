mod app;
mod ldd;
mod pacman;

use std::path::PathBuf;

use structopt::{clap, StructOpt};

use app::App;

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
#[structopt(setting(clap::AppSettings::DisableHelpSubcommand))]
enum CargoOpt {
    #[structopt(about = "Cargo subcommand to see linked packages.")]
    Linked(Opt),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "cargo_linked", about = "List dynamically linked files")]
struct Opt {
    #[structopt(name = "PATH", long = "manifest-path", parse(from_os_str))]
    /// Path to Cargo.toml.
    manifest_path: Option<PathBuf>,

    #[structopt(name = "CURRENT_DIR", long = "current-dir", parse(from_os_str))]
    /// Current directory of the cargo metadata process.
    current_dir: Option<PathBuf>,
}

fn main() {
    let CargoOpt::Linked(opt) = CargoOpt::from_args();
    let mut cmd = cargo_metadata::MetadataCommand::new();

    if let Some(path) = &opt.manifest_path {
        cmd.manifest_path(path);
    }
    if let Some(dir) = &opt.current_dir {
        cmd.current_dir(dir);
    }

    let exec = match cmd.exec() {
        Ok(e) => e,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    App::new(exec).run();
}
