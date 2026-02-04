//! Rebuild command - wraps `nh os switch`.

use std::path::Path;

use anyhow::{Context, Result};

use crate::cli::RebuildArgs;
use crate::env;
use crate::exec::CommandRunner;
use crate::flake::resolve_flake_path;
use crate::host::get_hostname;
use crate::output;

/// Execute the rebuild command.
pub fn run(args: &RebuildArgs, flake_path: Option<&Path>) -> Result<()> {
    let host = match &args.host {
        Some(h) => h.clone(),
        None => get_hostname().context("could not determine hostname for rebuild")?,
    };

    let flake = resolve_flake_path(flake_path)?;

    let build_host = if args.local {
        None
    } else {
        args.build_host.clone().or_else(env::get_build_host)
    };

    let extra_args = env::get_extra_args();

    output::info(&format!("Rebuilding configuration for host: {}", host));

    if let Some(ref bh) = build_host {
        output::status(&format!("Building on remote host: {}", bh));
    }

    let mut runner = CommandRunner::new("nh")
        .args(["os", "switch"])
        .arg(&flake)
        .args(["-H", &host]);

    if let Some(ref bh) = build_host {
        runner = runner.args(["--build-host", bh]);
    }
    if let Some(ref sub) = args.substituter {
        runner = runner.args(["--extra-substituters", sub]);
    }
    if let Some(ref key) = args.key {
        runner = runner.args(["--extra-trusted-public-keys", key]);
    }

    runner = runner.arg_if(args.trace, "--show-trace");
    runner = runner.arg_if(args.dry_run, "--dry-run");

    if !extra_args.is_empty() {
        runner = runner.arg("--").args(&extra_args);
    }

    runner.run()?;

    if args.dry_run {
        output::success("Dry run complete");
    } else {
        output::success("Rebuild complete!");
    }

    Ok(())
}
