//! Store repair command - wraps `nix store verify/repair`.

use anyhow::Result;

use crate::cli::store::RepairArgs;
use crate::exec::CommandRunner;
use crate::output;

/// Execute the store repair command.
pub fn run(args: &RepairArgs) -> Result<()> {
    if args.check_only {
        output::info("Verifying store integrity...");

        let mut runner = CommandRunner::new("nix").args(["store", "verify", "--all"]);

        if !args.paths.is_empty() {
            runner = CommandRunner::new("nix")
                .args(["store", "verify"])
                .args(&args.paths);
        }

        runner.run()?;
        output::success("Store verification complete!");
    } else {
        output::info("Verifying and repairing store...");
        output::status("Corrupted paths will be re-downloaded from caches...");

        let mut runner = CommandRunner::new("nix").args(["store", "repair", "--all"]);

        if !args.paths.is_empty() {
            runner = CommandRunner::new("nix")
                .args(["store", "repair"])
                .args(&args.paths);
        }

        runner.run()?;
        output::success("Store repair complete!");
    }

    Ok(())
}
