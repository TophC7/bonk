//! Build script for generating shell completions at compile time.
//!
//! This script generates shell completion files (fish, bash, zsh) during
//! `cargo build`. The completions are written to OUT_DIR and can be installed
//! by the Nix derivation using installShellFiles.
//!
//! ## How it works
//!
//! We create a module hierarchy that mirrors `src/cli/` exactly, allowing
//! `root.rs` to use its standard imports (`super::build::BuildArgs`, etc.)
//! without modification. This eliminates CLI definition duplication.

use std::env;
use std::fs;
use std::io::Error;
use std::path::PathBuf;

use clap::CommandFactory;
use clap_complete::{generate_to, Shell};

// Create a cli module structure that mirrors src/cli/.
// This allows root.rs's `super::submodule::Type` imports to resolve correctly.
#[path = "src/cli"]
mod cli {
    #[path = "build.rs"]
    pub mod build;
    #[path = "rebuild.rs"]
    pub mod rebuild;
    #[path = "root.rs"]
    mod root;
    #[path = "store.rs"]
    pub mod store;
    #[path = "try_pkg.rs"]
    pub mod try_pkg;
    #[path = "update.rs"]
    pub mod update;

    pub use root::Cli;
}

use cli::Cli;

fn main() -> Result<(), Error> {
    // Get the output directory from cargo
    let out_dir = match env::var_os("OUT_DIR") {
        Some(dir) => PathBuf::from(dir),
        None => return Ok(()), // Skip if OUT_DIR not set (shouldn't happen in normal builds)
    };

    // Create a completions subdirectory for organization
    let completions_dir = out_dir.join("completions");
    fs::create_dir_all(&completions_dir)?;

    // Build the clap Command from our CLI struct
    let mut cmd = Cli::command();

    // Generate completions for each supported shell.
    // Fish is the primary target, but we generate others for completeness.
    let shells = [Shell::Fish, Shell::Bash, Shell::Zsh];

    for shell in shells {
        generate_to(shell, &mut cmd, "bonk", &completions_dir)?;
    }

    // Tell cargo to rerun this script if CLI definitions change
    println!("cargo:rerun-if-changed=src/cli/");
    println!("cargo:rerun-if-changed=build.rs");

    // Export the completions directory path for the Nix build to find
    // This is written to a file that the Nix derivation can read
    let completions_path_file = out_dir.join("completions_dir.txt");
    fs::write(
        &completions_path_file,
        completions_dir.to_string_lossy().as_bytes(),
    )?;

    Ok(())
}
