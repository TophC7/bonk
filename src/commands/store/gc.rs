//! Store gc command - wraps `nh clean all`, then rebuilds boot entries.

use anyhow::Result;

use crate::cli::store::GcArgs;
use crate::exec::CommandRunner;
use crate::output;

/// Execute the store gc command.
///
/// Removes old NixOS generations via `nh clean all`, then rebuilds bootloader
/// entries so the boot menu reflects the surviving generations.
///
/// # Arguments
///
/// * `args` - CLI arguments controlling retention and dry-run behaviour
///
/// # Errors
///
/// Returns an error if `nh clean all` or the bootloader rebuild fails.
pub fn run(args: &GcArgs) -> Result<()> {
    if args.dry_run {
        output::info("Dry run: showing what would be garbage collected...");
    } else {
        output::info("Garbage collecting old generations...");
    }

    let mut runner = CommandRunner::new("nh").args(["clean", "all"]);

    runner = runner.args(["--keep", &args.keep.to_string()]);

    if let Some(ref duration) = args.older_than {
        runner = runner.args(["--keep-since", duration]);
    }

    runner = runner.arg_if(args.dry_run, "--dry-run");

    runner.run()?;

    // Rebuild bootloader entries so removed generations no longer appear
    // in the boot menu. Without this, stale entries persist until the next
    // `nixos-rebuild` or `switch-to-configuration boot`.
    if !args.dry_run {
        output::info("Rebuilding bootloader entries...");
        CommandRunner::new("sudo")
            .args(["/run/current-system/bin/switch-to-configuration", "boot"])
            .run()?;
    }

    if args.dry_run {
        output::success("Dry run complete");
    } else {
        output::success("Garbage collection complete!");
    }

    Ok(())
}
