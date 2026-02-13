//! Store nuke command - aggressive full cleanup.

use std::io::{self, Write};
use std::path::Path;

use anyhow::Result;

use crate::cli::os::OsArgs;
use crate::cli::store::NukeArgs;
use crate::commands::os::OsAction;
use crate::exec::CommandRunner;
use crate::output;

/// Two passes ensure transitively-freed store paths are caught.
const NUKE_PASSES: u32 = 2;

/// Execute the store nuke command.
///
/// Performs aggressive cleanup in two full passes, then rebuilds boot entries
/// so the system remains bootable. Pass `--no-rebuild` to skip the rebuild.
///
/// # Errors
///
/// Returns an error if any subprocess fails or if user input cannot be read.
pub fn run(args: &NukeArgs, flake_path: Option<&Path>) -> Result<()> {
    if !args.yes {
        output::warn("WARNING: This will perform aggressive cleanup:");
        println!("  - Rebuild bootloader entries (drop old generation GC roots)");
        println!("  - Remove ALL old generations (keeps only current)");
        println!("  - Run full garbage collection");
        println!("  - Optimize the store");
        println!("  - Run the full cycle twice to catch transitive references");
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

    if args.remove_results {
        output::info("Removing result symlinks (GC roots)...");
        remove_result_symlinks()?;
    }

    for pass in 1..=NUKE_PASSES {
        output::header(&format!("Pass {pass}/{NUKE_PASSES}"));
        run_cleanup_pass(pass, NUKE_PASSES)?;
    }

    if args.no_rebuild {
        output::warn("Skipping rebuild -- system may be unbootable until you rebuild manually!");
    } else {
        output::header("Rebuilding boot entries");
        crate::commands::os::run(OsAction::Boot, &OsArgs::default(), flake_path)?;
    }

    output::success("Nuke complete! Store is now clean and optimized.");

    Ok(())
}

/// Run a single cleanup pass (boot entries, clean, gc, optimise).
fn run_cleanup_pass(pass: u32, total: u32) -> Result<()> {
    output::header(&format!(
        "  [{pass}/{total}] Step 1/4: Rebuilding bootloader entries"
    ));
    CommandRunner::new("sudo")
        .args(["/run/current-system/bin/switch-to-configuration", "boot"])
        .run()?;

    output::header(&format!(
        "  [{pass}/{total}] Step 2/4: Removing old generations"
    ));
    CommandRunner::new("nh")
        .args(["clean", "all", "--keep", "0"])
        .run()?;

    output::header(&format!(
        "  [{pass}/{total}] Step 3/4: Garbage collecting store"
    ));
    CommandRunner::new("nix-collect-garbage").arg("-d").run()?;

    output::header(&format!("  [{pass}/{total}] Step 4/4: Optimizing store"));
    CommandRunner::new("nix")
        .args(["store", "optimise"])
        .run()?;

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
