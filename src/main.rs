//! bonk - NixOS workflow multitool.

mod cli;
mod commands;
mod env;
mod exec;
mod flake;
mod host;
mod output;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use cli::{Cli, Commands, StoreCommands};
use commands::os::OsAction;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let default_level = if cli.verbose { "info" } else { "warn" };
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level)),
        )
        .with_target(false)
        .without_time()
        .init();

    if cli.verbose {
        if let Some(ref path) = cli.flake_path {
            output::status(&format!("Using flake path: {}", path.display()));
        }
    }

    match cli.command {
        Commands::Switch(args) => {
            if cli.verbose {
                output::status("Running switch command");
            }
            commands::os::run(OsAction::Switch, &args, cli.flake_path.as_deref())?;
        }
        Commands::Boot(args) => {
            if cli.verbose {
                output::status("Running boot command");
            }
            commands::os::run(OsAction::Boot, &args, cli.flake_path.as_deref())?;
        }
        Commands::Build(args) => {
            if cli.verbose {
                output::status("Running build command");
            }
            commands::build::run(&args, cli.flake_path.as_deref())?;
        }
        Commands::Update(args) => {
            if cli.verbose {
                output::status("Running update command");
            }
            commands::update::run(&args, cli.flake_path.as_deref())?;
        }
        Commands::Try(args) => {
            if cli.verbose {
                output::status("Running try command");
            }
            commands::try_pkg::run(&args)?;
        }
        Commands::Store { command } => match command {
            StoreCommands::Gc(args) => {
                if cli.verbose {
                    output::status("Running store gc command");
                }
                commands::store::gc::run(&args)?;
            }
            StoreCommands::Optimize(args) => {
                if cli.verbose {
                    output::status("Running store optimize command");
                }
                commands::store::optimize::run(&args)?;
            }
            StoreCommands::Repair(args) => {
                if cli.verbose {
                    output::status("Running store repair command");
                }
                commands::store::repair::run(&args)?;
            }
            StoreCommands::Nuke(args) => {
                if cli.verbose {
                    output::status("Running store nuke command");
                }
                commands::store::nuke::run(&args, cli.flake_path.as_deref())?;
            }
            StoreCommands::Info(args) => {
                if cli.verbose {
                    output::status("Running store info command");
                }
                commands::store::info::run(&args)?;
            }
        },
    }

    Ok(())
}
