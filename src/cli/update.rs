//! Update command arguments.

use clap::Parser;

#[derive(Parser, Debug)]
pub struct UpdateArgs {
    /// Specific inputs to update (all if empty).
    #[arg()]
    pub inputs: Vec<String>,

    /// Commit the lock file changes.
    #[arg(short, long)]
    pub commit: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse(args: &[&str]) -> UpdateArgs {
        #[derive(Parser)]
        struct Cli {
            #[command(flatten)]
            update: UpdateArgs,
        }
        let mut full = vec!["test"];
        full.extend(args);
        Cli::try_parse_from(full).unwrap().update
    }

    #[test]
    fn test_default_args() {
        let args = parse(&[]);
        assert!(args.inputs.is_empty());
        assert!(!args.commit);
    }

    #[test]
    fn test_single_input() {
        assert_eq!(parse(&["nixpkgs"]).inputs, vec!["nixpkgs"]);
    }

    #[test]
    fn test_multiple_inputs() {
        assert_eq!(
            parse(&["nixpkgs", "home-manager"]).inputs,
            vec!["nixpkgs", "home-manager"]
        );
    }

    #[test]
    fn test_commit_flag() {
        assert!(parse(&["--commit"]).commit);
    }
}
