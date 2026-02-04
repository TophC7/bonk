//! Store command arguments.

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum StoreCommands {
    /// Garbage collect old generations.
    #[command(name = "gc", alias = "clean")]
    Gc(GcArgs),

    /// Deduplicate store via hard-linking.
    #[command(name = "optimize")]
    Optimize(OptimizeArgs),

    /// Verify and repair store integrity.
    #[command(name = "repair")]
    Repair(RepairArgs),

    /// Aggressive full cleanup (removes all old generations).
    #[command(name = "nuke")]
    Nuke(NukeArgs),

    /// Show store statistics.
    #[command(name = "info")]
    Info(InfoArgs),
}

#[derive(Parser, Debug)]
pub struct GcArgs {
    /// Delete generations older than this (e.g., 7d, 2w, 1m).
    #[arg(short, long)]
    pub older_than: Option<String>,

    /// Keep at least this many generations.
    #[arg(short, long, default_value = "3")]
    pub keep: u32,

    /// Show what would be deleted without deleting.
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[derive(Parser, Debug)]
pub struct OptimizeArgs {
    /// Show potential savings without optimizing.
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[derive(Parser, Debug)]
pub struct RepairArgs {
    /// Specific store paths to repair (all if empty).
    #[arg()]
    pub paths: Vec<String>,

    /// Only verify, don't repair.
    #[arg(short, long)]
    pub check_only: bool,
}

#[derive(Parser, Debug)]
pub struct NukeArgs {
    /// Skip confirmation prompt.
    #[arg(short, long)]
    pub yes: bool,

    /// Also remove result symlinks in current directory.
    #[arg(short, long)]
    pub remove_results: bool,
}

#[derive(Parser, Debug)]
pub struct InfoArgs {
    /// Show detailed breakdown.
    #[arg(short, long)]
    pub detailed: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[derive(Parser)]
    struct Cli {
        #[command(subcommand)]
        command: StoreCommands,
    }

    fn parse(args: &[&str]) -> StoreCommands {
        let mut full = vec!["test"];
        full.extend(args);
        Cli::try_parse_from(full).unwrap().command
    }

    #[test]
    fn test_gc_default() {
        match parse(&["gc"]) {
            StoreCommands::Gc(args) => {
                assert!(args.older_than.is_none());
                assert_eq!(args.keep, 3);
                assert!(!args.dry_run);
            }
            _ => panic!("expected gc"),
        }
    }

    #[test]
    fn test_gc_with_older_than() {
        match parse(&["gc", "--older-than", "7d"]) {
            StoreCommands::Gc(args) => assert_eq!(args.older_than, Some("7d".to_string())),
            _ => panic!("expected gc"),
        }
    }

    #[test]
    fn test_gc_alias_clean() {
        assert!(matches!(parse(&["clean"]), StoreCommands::Gc(_)));
    }

    #[test]
    fn test_optimize() {
        assert!(matches!(parse(&["optimize"]), StoreCommands::Optimize(_)));
    }

    #[test]
    fn test_repair_check_only() {
        match parse(&["repair", "--check-only"]) {
            StoreCommands::Repair(args) => assert!(args.check_only),
            _ => panic!("expected repair"),
        }
    }

    #[test]
    fn test_nuke_with_yes() {
        match parse(&["nuke", "-y"]) {
            StoreCommands::Nuke(args) => assert!(args.yes),
            _ => panic!("expected nuke"),
        }
    }

    #[test]
    fn test_info() {
        assert!(matches!(parse(&["info"]), StoreCommands::Info(_)));
    }
}
