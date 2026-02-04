//! Rebuild command arguments.

use clap::Parser;

#[derive(Parser, Debug)]
pub struct RebuildArgs {
    /// Target host (defaults to current hostname).
    #[arg(short = 'H', long)]
    pub host: Option<String>,

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

    fn parse(args: &[&str]) -> RebuildArgs {
        #[derive(Parser)]
        struct Cli {
            #[command(flatten)]
            rebuild: RebuildArgs,
        }
        let mut full = vec!["test"];
        full.extend(args);
        Cli::try_parse_from(full).unwrap().rebuild
    }

    #[test]
    fn test_default_args() {
        let args = parse(&[]);
        assert!(args.host.is_none());
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
