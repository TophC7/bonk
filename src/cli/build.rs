//! Build command arguments.

use clap::Parser;

#[derive(Parser, Debug)]
pub struct BuildArgs {
    /// Package or flake output to build (default package if empty).
    #[arg()]
    pub target: Option<String>,

    /// Don't create the result symlink.
    #[arg(long)]
    pub no_link: bool,

    /// Output path for the result symlink.
    #[arg(short, long)]
    pub out_link: Option<String>,

    /// Enable --show-trace for debugging.
    #[arg(short, long)]
    pub trace: bool,

    /// Show what would be built without building.
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse(args: &[&str]) -> BuildArgs {
        #[derive(Parser)]
        struct Cli {
            #[command(flatten)]
            build: BuildArgs,
        }
        let mut full = vec!["test"];
        full.extend(args);
        Cli::try_parse_from(full).unwrap().build
    }

    #[test]
    fn test_default_args() {
        let args = parse(&[]);
        assert!(args.target.is_none());
        assert!(!args.no_link);
        assert!(args.out_link.is_none());
        assert!(!args.trace);
        assert!(!args.dry_run);
    }

    #[test]
    fn test_target() {
        assert_eq!(parse(&[".#pkg"]).target, Some(".#pkg".to_string()));
    }

    #[test]
    fn test_no_link() {
        assert!(parse(&["--no-link"]).no_link);
    }

    #[test]
    fn test_out_link() {
        assert_eq!(
            parse(&["-o", "my-result"]).out_link,
            Some("my-result".to_string())
        );
    }

    #[test]
    fn test_trace() {
        assert!(parse(&["-t"]).trace);
    }
}
