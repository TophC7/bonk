//! OS commands - wraps `nh os switch` and `nh os boot`.

use std::path::Path;

use anyhow::{Context, Result};

use crate::cli::OsArgs;
use crate::env;
use crate::exec::CommandRunner;
use crate::flake::resolve_flake_path;
use crate::host::get_hostname;
use crate::output;

/// The nh os action to perform.
#[derive(Debug, Clone, Copy)]
pub enum OsAction {
    Switch,
    Boot,
}

impl OsAction {
    /// Return the nh subcommand string.
    fn as_str(self) -> &'static str {
        match self {
            OsAction::Switch => "switch",
            OsAction::Boot => "boot",
        }
    }
}

/// Execute an os rebuild with the given action.
///
/// # Arguments
///
/// * `action` - Whether to `switch` (activate now) or `boot` (next boot only)
/// * `args` - CLI arguments shared by both switch and boot
/// * `flake_path` - Optional explicit flake path override
///
/// # Errors
///
/// Returns an error if hostname detection, flake resolution, or the nh command fails.
pub fn run(action: OsAction, args: &OsArgs, flake_path: Option<&Path>) -> Result<()> {
    // Resolve which NixOS flake configuration to build (-H).
    let host = match &args.host {
        Some(h) => h.clone(),
        None => get_hostname().context("could not determine hostname for rebuild")?,
    };

    // Resolve where to deploy (-T / --target-host):
    //  -TH zebes           -> deploy to the -H host (zebes)
    //  --target-host addr   -> deploy to a specific SSH address
    //  neither              -> local deploy (no --target-host passed to nh)
    let deploy_target = if let Some(ref th) = args.target_host {
        Some(th.clone())
    } else if args.target {
        Some(host.clone())
    } else {
        None
    };

    let flake = resolve_flake_path(flake_path)?;

    let build_host = if args.local {
        None
    } else {
        args.build_host.clone().or_else(env::get_build_host)
    };

    let extra_args = env::get_extra_args();
    let label = action.as_str();

    output::info(&format!(
        "Rebuilding configuration for host: {} ({})",
        host, label
    ));

    if let Some(ref dt) = deploy_target {
        output::status(&format!("Deploying to target host: {}", dt));
    }
    if let Some(ref bh) = build_host {
        output::status(&format!("Building on remote host: {}", bh));
    }

    let mut runner = CommandRunner::new("nh")
        .args(["os", label])
        .arg(&flake)
        .args(["-H", &host]);

    if let Some(ref dt) = deploy_target {
        runner = runner.args(["--target-host", dt]);
    }
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
        output::success(&format!("Dry run complete ({})", label));
    } else {
        output::success(&format!("Rebuild complete! ({})", label));
    }

    Ok(())
}
