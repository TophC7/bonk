//! Shared arguments for `switch` and `boot` commands.

use clap::Parser;

#[derive(Parser, Debug)]
pub struct OsArgs {
    /// Target host (defaults to current hostname).
    #[arg(short = 'H', long)]
    pub host: Option<String>,

    /// Deploy to a remote host via SSH (e.g. root@192.168.1.50).
    /// Use when the target machine doesn't yet have the expected hostname.
    #[arg(short = 'T', long)]
    pub target_host: Option<String>,

    /// Build on a remote host instead of locally.
    #[arg(short = 'B', long)]
    pub build_host: Option<String>,

    /// Force local build, ignoring BONK_BUILD_HOST.
    #[arg(short, long)]
    pub local: bool,

    /// Enable --show-trace for debugging.
    #[arg(short, long)]
    pub trace: bool,

    /// Extra binary cache URL.
    #[arg(short = 's', long)]
    pub substituter: Option<String>,

    /// Trusted public key for the cache.
    #[arg(short = 'k', long)]
    pub key: Option<String>,

    /// Show what would be built without building.
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse(args: &[&str]) -> OsArgs {
        #[derive(Parser)]
        struct Cli {
            #[command(flatten)]
            os: OsArgs,
        }
        let mut full = vec!["test"];
        full.extend(args);
        Cli::try_parse_from(full).unwrap().os
    }

    #[test]
    fn test_default_args() {
        let args = parse(&[]);
        assert!(args.host.is_none());
        assert!(args.target_host.is_none());
        assert!(args.build_host.is_none());
        assert!(!args.local);
        assert!(!args.trace);
        assert!(!args.dry_run);
    }

    #[test]
    fn test_host_flag() {
        let args = parse(&["-H", "rune"]);
        assert_eq!(args.host, Some("rune".to_string()));
    }

    #[test]
    fn test_target_host_flag() {
        let args = parse(&["-T", "root@192.168.1.50"]);
        assert_eq!(args.target_host, Some("root@192.168.1.50".to_string()));
    }

    #[test]
    fn test_build_host_flag() {
        let args = parse(&["-B", "buildserver"]);
        assert_eq!(args.build_host, Some("buildserver".to_string()));
    }

    #[test]
    fn test_local_flag() {
        assert!(parse(&["--local"]).local);
    }

    #[test]
    fn test_trace_flag() {
        assert!(parse(&["-t"]).trace);
    }

    #[test]
    fn test_substituter_and_key() {
        let args = parse(&["-s", "https://cache.example.com", "-k", "key:AAAA..."]);
        assert_eq!(
            args.substituter,
            Some("https://cache.example.com".to_string())
        );
        assert_eq!(args.key, Some("key:AAAA...".to_string()));
    }

    #[test]
    fn test_dry_run_flag() {
        assert!(parse(&["-n"]).dry_run);
    }
}
