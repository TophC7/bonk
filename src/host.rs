//! Hostname detection utilities.

use anyhow::{Context, Result};

/// Get the system hostname.
pub fn get_hostname() -> Result<String> {
    hostname::get()
        .context("failed to get system hostname")?
        .into_string()
        .map_err(|_| anyhow::anyhow!("hostname contains invalid UTF-8"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hostname_returns_non_empty() {
        let hostname = get_hostname().expect("should get hostname");
        assert!(!hostname.is_empty());
    }
}
