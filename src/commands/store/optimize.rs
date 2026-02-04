//! Store optimize command - wraps `nix store optimise`.

use anyhow::Result;

use crate::cli::store::OptimizeArgs;
use crate::exec::CommandRunner;
use crate::output;

/// Execute the store optimize command.
pub fn run(args: &OptimizeArgs) -> Result<()> {
    if args.dry_run {
        output::info("Dry run: analyzing store for optimization opportunities...");
        output::warn("Note: nix store optimise does not support dry-run directly.");
        output::warn("Running du to estimate store size...");

        CommandRunner::new("du")
            .args(["-sh", "/nix/store"])
            .show_command(false)
            .run()?;

        output::info("Run without --dry-run to actually optimize.");
        return Ok(());
    }

    output::info("Optimizing Nix store (deduplicating via hard-links)...");
    output::status("This may take a while for large stores...");

    CommandRunner::new("nix")
        .args(["store", "optimise"])
        .run()?;

    output::success("Store optimization complete!");

    Ok(())
}
