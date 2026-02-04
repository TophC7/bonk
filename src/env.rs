//! Environment variable configuration.
//!
//! | Variable          | Purpose                            |
//! |-------------------|------------------------------------|
//! | `BONK_FLAKE_PATH` | Default flake path                 |
//! | `BONK_BUILD_HOST` | Default build host (empty = local) |
//! | `BONK_EXTRA_ARGS` | Extra args (colon-separated)       |

use std::env;
use std::path::PathBuf;

/// Get flake path from environment.
pub fn get_flake_path() -> Option<PathBuf> {
    env::var("BONK_FLAKE_PATH")
        .ok()
        .or_else(|| env::var("FLAKE").ok())
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
}

/// Get default build host from environment.
pub fn get_build_host() -> Option<String> {
    env::var("BONK_BUILD_HOST").ok().filter(|s| !s.is_empty())
}

/// Get extra args from environment.
pub fn get_extra_args() -> Vec<String> {
    env::var("BONK_EXTRA_ARGS")
        .ok()
        .map(|s| {
            s.split(':')
                .filter(|arg| !arg.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_get_flake_path_none_when_unset() {
        env::remove_var("BONK_FLAKE_PATH");
        env::remove_var("FLAKE");
        assert!(get_flake_path().is_none());
    }

    #[test]
    #[serial]
    fn test_get_extra_args_empty_when_unset() {
        env::remove_var("BONK_EXTRA_ARGS");
        assert!(get_extra_args().is_empty());
    }
}
