//! CLI argument definitions.
//!
//! This module defines the command-line interface using clap's derive macros.
//! The CLI structure is publicly exported via lib.rs for build-time tooling
//! (shell completion generation, man page generation, etc.).

// All modules are public so lib.rs consumers can access types for codegen
pub mod build;
pub mod os;
pub mod root;
pub mod store;
pub mod try_pkg;
pub mod update;

pub use build::BuildArgs;
pub use os::OsArgs;
pub use root::{Cli, Commands};
pub use store::StoreCommands;
pub use try_pkg::TryArgs;
pub use update::UpdateArgs;
