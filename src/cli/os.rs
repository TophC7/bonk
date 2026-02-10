//! Shared arguments for `switch` and `boot` commands.

use clap::Parser;

#[derive(Parser, Debug)]
pub struct OsArgs {
    /// NixOS flake configuration to build (e.g. `zebes` selects
    /// `nixosConfigurations.zebes`). Defaults to the current hostname.
    /// Does NOT control where the result is deployed -- use `-T` for that.
    #[arg(short = 'H', long)]
    pub host: Option<String>,

    /// Also deploy the built configuration to the `-H` host via SSH.
    /// Combine as `-TH <host>` to select and deploy in one shot.
    #[arg(short = 'T', long)]
    pub target: bool,

    /// Deploy to a specific SSH target that differs from `-H`
    /// (e.g. `root@192.168.1.50` for a machine not yet renamed).
    #[arg(long)]
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
        assert!(!args.target);
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
    fn test_combined_target_host() {
        // -TH zebes: -T (bool) + -H zebes (value)
        let args = parse(&["-TH", "zebes"]);
        assert!(args.target);
        assert_eq!(args.host, Some("zebes".to_string()));
    }

    #[test]
    fn test_target_flag_alone() {
        let args = parse(&["-T"]);
        assert!(args.target);
        assert!(args.host.is_none());
    }

    #[test]
    fn test_target_host_long() {
        let args = parse(&["--target-host", "root@192.168.1.50"]);
        assert_eq!(args.target_host, Some("root@192.168.1.50".to_string()));
        assert!(!args.target);
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
