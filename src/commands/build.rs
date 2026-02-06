//! Build command - wraps `nix build`.

use std::path::Path;

use anyhow::Result;

use crate::cli::BuildArgs;
use crate::env;
use crate::exec::CommandRunner;
use crate::flake::resolve_flake_path;
use crate::output;

/// Execute the build command.
pub fn run(args: &BuildArgs, flake_path: Option<&Path>) -> Result<()> {
    let target = match &args.target {
        Some(t) => t.clone(),
        None => resolve_flake_path(flake_path)?,
    };

    // Resolve build host: --local disables, --build-host overrides, else env fallback
    let build_host = if args.local {
        None
    } else {
        args.build_host.clone().or_else(env::get_build_host)
    };

    output::info(&format!("Building: {}", target));

    if let Some(ref bh) = build_host {
        output::status(&format!("Building on remote host: {}", bh));
    }

    let mut runner = CommandRunner::new("nix").arg("build");

    runner = runner.arg(&target);

    // Configure remote builder if specified
    // --max-jobs 0 forces all builds to go to the remote builder
    if let Some(ref bh) = build_host {
        let builder_spec = format!("ssh://{}", bh);
        runner = runner.args(["--builders", &builder_spec, "--max-jobs", "0"]);
    }

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
