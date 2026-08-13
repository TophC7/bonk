//! Development shell command arguments.

use clap::Parser;

/// Arguments for entering a flake development shell.
#[derive(Parser, Debug)]
pub struct DevArgs {
    /// Named dev shell (uses the flake's default shell when omitted).
    #[arg()]
    pub shell: Option<String>,

    /// Use Nix's default Bash instead of the user's login shell.
    #[arg(long)]
    pub bash: bool,

    /// Allow access to mutable paths and repositories.
    #[arg(long)]
    pub impure: bool,

    /// Command to run instead of opening an interactive shell.
    #[arg(last = true, num_args = 0..)]
    pub command: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse(args: &[&str]) -> DevArgs {
        #[derive(Parser)]
        struct Cli {
            #[command(flatten)]
            dev: DevArgs,
        }

        let mut full = vec!["test"];
        full.extend(args);
        Cli::try_parse_from(full)
            .expect("development shell arguments should parse")
            .dev
    }

    #[test]
    fn default_args() {
        let args = parse(&[]);
        assert!(args.shell.is_none());
        assert!(!args.bash);
        assert!(!args.impure);
        assert!(args.command.is_empty());
    }

    #[test]
    fn named_shell() {
        assert_eq!(parse(&["frontend"]).shell.as_deref(), Some("frontend"));
    }

    #[test]
    fn bash_and_impure() {
        let args = parse(&["--bash", "--impure"]);
        assert!(args.bash);
        assert!(args.impure);
    }

    #[test]
    fn trailing_command() {
        let args = parse(&["frontend", "--", "cargo", "test"]);
        assert_eq!(args.shell.as_deref(), Some("frontend"));
        assert_eq!(args.command, ["cargo", "test"]);
    }

    #[test]
    fn command_without_named_shell() {
        let args = parse(&["--", "cargo", "test"]);
        assert!(args.shell.is_none());
        assert_eq!(args.command, ["cargo", "test"]);
    }
}
