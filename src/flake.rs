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
    if let Some(flake_dir) = find_nearest_flake(&current_dir) {
        if flake_dir == current_dir {
            return Ok(".".to_owned());
        }
        return Ok(flake_dir.display().to_string());
    }

    if let Some(env_path) = env::get_flake_path() {
        return Ok(env_path.display().to_string());
    }

    anyhow::bail!(
        "no flake path found. Either:\n\
         - Run within a directory tree containing flake.nix\n\
         - Set BONK_FLAKE_PATH environment variable\n\
         - Use --flake-path / -p option"
    )
}

fn find_nearest_flake(start: &Path) -> Option<std::path::PathBuf> {
    start
        .ancestors()
        .find(|directory| directory.join("flake.nix").is_file())
        .map(Path::to_path_buf)
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

    #[test]
    fn finds_flake_in_nearest_ancestor() {
        let root = tempfile::tempdir().expect("temporary directory should be created");
        let nested = root.path().join("one/two");
        std::fs::create_dir_all(&nested).expect("temporary directories should be created");
        std::fs::write(root.path().join("flake.nix"), "{}")
            .expect("temporary flake should be written");

        assert_eq!(find_nearest_flake(&nested).as_deref(), Some(root.path()));
    }
}
