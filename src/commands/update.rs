//! Update command - wraps `nix flake update`.

use std::path::Path;

use anyhow::Result;

use crate::cli::UpdateArgs;
use crate::exec::CommandRunner;
use crate::flake::resolve_flake_path;
use crate::output;

/// Execute the update command.
pub fn run(args: &UpdateArgs, flake_path: Option<&Path>) -> Result<()> {
    let flake = resolve_flake_path(flake_path)?;

    if args.inputs.is_empty() {
        output::info("Updating all flake inputs...");
    } else {
        output::info(&format!("Updating inputs: {}", args.inputs.join(", ")));
    }

    let mut runner = CommandRunner::new("nix").args(["flake", "update", "--flake", &flake]);

    // Add specific inputs to update (if none specified, all inputs are updated)
    for input in &args.inputs {
        runner = runner.arg(input);
    }

    runner = runner.arg_if(args.commit, "--commit-lock-file");

    runner.run()?;

    output::success("Update complete!");

    Ok(())
}
