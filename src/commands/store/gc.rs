//! Store gc command - wraps `nh clean all`.

use anyhow::Result;

use crate::cli::store::GcArgs;
use crate::exec::CommandRunner;
use crate::output;

/// Execute the store gc command.
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

    if args.dry_run {
        output::success("Dry run complete");
    } else {
        output::success("Garbage collection complete!");
    }

    Ok(())
}
