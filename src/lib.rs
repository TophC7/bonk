//! bonk library - exposes CLI structure for build-time tooling.
//!
//! This module re-exports the CLI definition so that build scripts
//! can generate shell completions, man pages, etc. at compile time.

pub mod cli;

// Re-export the main CLI types for convenience
pub use cli::Cli;
