//! Try command arguments.

use clap::Parser;

#[derive(Parser, Debug)]
pub struct TryArgs {
    /// Packages to make available (resolved from nixpkgs by default).
    #[arg(required = true, num_args = 1..)]
    pub packages: Vec<String>,

    /// Command to run (after --). Opens interactive shell if empty.
    #[arg(last = true, num_args = 0..)]
    pub cmd: Vec<String>,

    /// Use a pure shell (no inherited environment).
    #[arg(long)]
    pub pure: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse(args: &[&str]) -> TryArgs {
        #[derive(Parser)]
        struct Cli {
            #[command(flatten)]
            try_args: TryArgs,
        }
        let mut full = vec!["test"];
        full.extend(args);
        Cli::try_parse_from(full).unwrap().try_args
    }

    #[test]
    fn test_single_package() {
        let args = parse(&["ripgrep"]);
        assert_eq!(args.packages, vec!["ripgrep"]);
        assert!(args.cmd.is_empty());
        assert!(!args.pure);
    }

    #[test]
    fn test_multiple_packages() {
        assert_eq!(
            parse(&["ripgrep", "fd", "bat"]).packages,
            vec!["ripgrep", "fd", "bat"]
        );
    }

    #[test]
    fn test_with_command() {
        let args = parse(&["cowsay", "--", "cowsay", "moo"]);
        assert_eq!(args.packages, vec!["cowsay"]);
        assert_eq!(args.cmd, vec!["cowsay", "moo"]);
    }

    #[test]
    fn test_pure_flag() {
        assert!(parse(&["python3", "--pure"]).pure);
    }

    #[test]
    fn test_pure_with_command() {
        let args = parse(&["python3", "--pure", "--", "python", "-c", "print('hi')"]);
        assert_eq!(args.packages, vec!["python3"]);
        assert!(args.pure);
        assert_eq!(args.cmd, vec!["python", "-c", "print('hi')"]);
    }
}
