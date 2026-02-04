//! Flake path resolution.

use std::path::Path;

use anyhow::{Context, Result};

use crate::env;

/// Resolve flake path.
pub fn resolve_flake_path(explicit_path: Option<&Path>) -> Result<String> {
    if let Some(path) = explicit_path {
        return Ok(path.display().to_string());
    }

    let current_dir = std::env::current_dir().context("failed to get current directory")?;
    if current_dir.join("flake.nix").exists() {
        return Ok(".".to_string());
    }

    if let Some(env_path) = env::get_flake_path() {
        return Ok(env_path.display().to_string());
    }

    anyhow::bail!(
        "no flake path found. Either:\n\
         - Run from a directory containing flake.nix\n\
         - Set BONK_FLAKE_PATH environment variable\n\
         - Use --flake-path / -p option"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_flake_path_explicit() {
        let result = resolve_flake_path(Some(Path::new("/some/path"))).unwrap();
        assert_eq!(result, "/some/path");
    }

    #[test]
    fn test_resolve_flake_path_explicit_relative() {
        let result = resolve_flake_path(Some(Path::new("./relative/path"))).unwrap();
        assert_eq!(result, "./relative/path");
    }
}
