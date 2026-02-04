//! Store nuke command - aggressive full cleanup.

use std::io::{self, Write};

use anyhow::Result;

use crate::cli::store::NukeArgs;
use crate::exec::CommandRunner;
use crate::output;

/// Execute the store nuke command.
///
/// Performs aggressive cleanup:
/// 1. Removes all old generations (keeps only current)
/// 2. Runs full garbage collection
/// 3. Optimizes the store
/// 4. Optionally removes result symlinks
pub fn run(args: &NukeArgs) -> Result<()> {
    if !args.yes {
        output::warn("WARNING: This will perform aggressive cleanup:");
        println!("  - Remove ALL old generations (keeps only current)");
        println!("  - Run full garbage collection");
        println!("  - Optimize the store");
        if args.remove_results {
            println!("  - Remove result symlinks in current directory");
        }
        println!();

        print!("Are you sure? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            output::info("Cancelled.");
            return Ok(());
        }
    }

    output::header("Step 1/3: Removing old generations");
    CommandRunner::new("nh")
        .args(["clean", "all", "--keep", "0"])
        .run()?;

    output::header("Step 2/3: Garbage collecting store");
    CommandRunner::new("nix-store").arg("--gc").run()?;

    output::header("Step 3/3: Optimizing store");
    CommandRunner::new("nix")
        .args(["store", "optimise"])
        .run()?;

    if args.remove_results {
        output::info("Removing result symlinks...");
        remove_result_symlinks()?;
    }

    output::success("Nuke complete! Store is now clean and optimized.");

    Ok(())
}

fn remove_result_symlinks() -> Result<()> {
    let current_dir = std::env::current_dir()?;

    for entry in std::fs::read_dir(&current_dir)?.filter_map(Result::ok) {
        let path = entry.path();

        if !path.is_symlink() {
            continue;
        }

        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name == "result" || name.starts_with("result-") {
                if let Ok(target) = std::fs::read_link(&path) {
                    if target.starts_with("/nix/store") {
                        output::status(&format!("Removing: {}", name));
                        std::fs::remove_file(&path)?;
                    }
                }
            }
        }
    }

    Ok(())
}
