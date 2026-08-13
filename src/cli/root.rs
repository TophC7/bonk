//! Root CLI structure and global arguments.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

// Use explicit submodule paths for build.rs compatibility.
// build.rs mirrors this structure so these paths resolve correctly there too.
use super::build::BuildArgs;
use super::dev::DevArgs;
use super::os::OsArgs;
use super::store::StoreCommands;
use super::try_pkg::TryArgs;
use super::update::UpdateArgs;

/// NixOS workflow multitool - wraps nh, nix, and nix-store.
#[derive(Parser)]
#[command(name = "bonk", version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Override the flake path.
    #[arg(short = 'p', long = "flake-path", global = true)]
    pub flake_path: Option<PathBuf>,

    /// Enable verbose output.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build and activate NixOS configuration now.
    #[command(name = "switch", alias = "s")]
    Switch(OsArgs),

    /// Build NixOS configuration and add boot entry without switching.
    #[command(name = "boot")]
    Boot(OsArgs),

    /// Build packages into the Nix store.
    #[command(name = "build", alias = "b")]
    Build(BuildArgs),

    /// Update flake inputs.
    #[command(name = "update", alias = "u")]
    Update(UpdateArgs),

    /// Enter a flake development shell.
    #[command(name = "dev")]
    Dev(DevArgs),

    /// Nix store management commands.
    #[command(name = "store")]
    Store {
        #[command(subcommand)]
        command: StoreCommands,
    },

    /// Create a temporary shell with packages.
    #[command(name = "try")]
    Try(TryArgs),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    struct FlakePathGuard(Option<std::ffi::OsString>);

    impl Drop for FlakePathGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.0 {
                std::env::set_var("BONK_FLAKE_PATH", value);
            } else {
                std::env::remove_var("BONK_FLAKE_PATH");
            }
        }
    }

    #[test]
    fn test_cli_parsing_switch() {
        let cli = Cli::try_parse_from(["bonk", "switch"]).unwrap();
        assert!(matches!(cli.command, Commands::Switch(_)));
    }

    #[test]
    fn test_cli_parsing_switch_alias() {
        let cli = Cli::try_parse_from(["bonk", "s"]).unwrap();
        assert!(matches!(cli.command, Commands::Switch(_)));
    }

    #[test]
    fn test_cli_parsing_boot() {
        let cli = Cli::try_parse_from(["bonk", "boot"]).unwrap();
        assert!(matches!(cli.command, Commands::Boot(_)));
    }

    #[test]
    fn test_cli_parsing_dev() {
        let cli = Cli::try_parse_from(["bonk", "dev", "frontend", "--impure"]).unwrap();
        let Commands::Dev(args) = cli.command else {
            panic!("expected dev command");
        };
        assert_eq!(args.shell.as_deref(), Some("frontend"));
        assert!(args.impure);
    }

    #[test]
    fn test_cli_parsing_with_flake_path() {
        let cli = Cli::try_parse_from(["bonk", "-p", "/path/to/flake", "switch"]).unwrap();
        assert_eq!(cli.flake_path, Some(PathBuf::from("/path/to/flake")));
    }

    #[test]
    fn test_cli_parsing_verbose() {
        let cli = Cli::try_parse_from(["bonk", "-v", "switch"]).unwrap();
        assert!(cli.verbose);
    }

    #[test]
    #[serial]
    fn environment_flake_path_remains_a_resolver_fallback() {
        let _guard = FlakePathGuard(std::env::var_os("BONK_FLAKE_PATH"));
        std::env::set_var("BONK_FLAKE_PATH", "/configured/flake");

        let cli = Cli::try_parse_from(["bonk", "switch"]).unwrap();

        assert!(cli.flake_path.is_none());
    }

    #[test]
    fn test_cli_parsing_store_gc() {
        let cli = Cli::try_parse_from(["bonk", "store", "gc"]).unwrap();
        assert!(matches!(
            cli.command,
            Commands::Store {
                command: StoreCommands::Gc(_)
            }
        ));
    }
}
