//! Build command - wraps `nix build`.

use std::path::Path;

use anyhow::Result;

use crate::cli::BuildArgs;
use crate::exec::CommandRunner;
use crate::flake::resolve_flake_path;
use crate::output;

/// Execute the build command.
pub fn run(args: &BuildArgs, flake_path: Option<&Path>) -> Result<()> {
    let target = match &args.target {
        Some(t) => t.clone(),
        None => resolve_flake_path(flake_path)?,
    };

    output::info(&format!("Building: {}", target));

    let mut runner = CommandRunner::new("nix").arg("build");

    runner = runner.arg(&target);

    if args.no_link {
        runner = runner.arg("--no-link");
    } else if let Some(ref out) = args.out_link {
        runner = runner.args(["-o", out]);
    }

    runner = runner.arg_if(args.trace, "--show-trace");
    runner = runner.arg_if(args.dry_run, "--dry-run");

    runner.run()?;

    if args.dry_run {
        output::success("Dry run complete");
    } else {
        output::success("Build complete!");
    }

    Ok(())
}
