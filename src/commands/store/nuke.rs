//! Store nuke command - aggressive full cleanup.
//!
//! Runs a two-pass cleanup cycle to ensure all old generations and their
//! store paths are fully removed. Each pass:
//!   1. Rebuilds bootloader entries (releases GC roots for old generations)
//!   2. Removes old generation profiles
//!   3. Garbage collects unreferenced store paths
//!   4. Deduplicates the store via hard-linking
//!
//! The second pass catches store paths that only become unreferenced after
//! the first pass removes intermediate references.

use std::io::{self, Write};

use anyhow::Result;

use crate::cli::store::NukeArgs;
use crate::exec::CommandRunner;
use crate::output;

/// Number of full cleanup passes to run.
///
/// Two passes ensure transitively-freed store paths are caught: the first
/// pass may free paths that were only indirectly referenced, and the second
/// pass garbage-collects those newly unreferenced paths.
const NUKE_PASSES: u32 = 2;

/// Execute the store nuke command.
///
/// Performs aggressive cleanup in two full passes:
/// 1. Rebuilds bootloader entries via `switch-to-configuration boot`
///    (drops GC roots the bootloader holds for old system generations)
/// 2. Removes all old generation profiles
/// 3. Garbage collects unreferenced store paths
/// 4. Optimizes (deduplicates) the store
///
/// The entire cycle runs twice to catch transitively-freed paths.
/// Optionally removes `result` symlinks in the current directory.
///
/// # Errors
///
/// Returns an error if any subprocess fails or if user input cannot be read.
pub fn run(args: &NukeArgs) -> Result<()> {
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

    for pass in 1..=NUKE_PASSES {
        output::header(&format!("Pass {pass}/{NUKE_PASSES}"));
        run_cleanup_pass(pass, NUKE_PASSES)?;
    }

    if args.remove_results {
        output::info("Removing result symlinks...");
        remove_result_symlinks()?;
    }

    output::success("Nuke complete! Store is now clean and optimized.");

    Ok(())
}

/// Run a single cleanup pass.
///
/// Each pass performs four steps:
/// 1. `switch-to-configuration boot` -- rebuilds bootloader entries so only
///    the current generation is referenced, releasing GC roots that previously
///    pinned old generation store paths.
/// 2. `nh clean all --keep 0` -- deletes old generation profile symlinks.
/// 3. `nix-collect-garbage -d` -- deletes any remaining old generations and
///    garbage collects all unreferenced store paths.
/// 4. `nix store optimise` -- deduplicates the store via hard-linking.
///
/// # Arguments
///
/// * `pass` - Current pass number (1-indexed, for display purposes)
/// * `total` - Total number of passes (for display purposes)
///
/// # Errors
///
/// Returns an error if any subprocess fails.
fn run_cleanup_pass(pass: u32, total: u32) -> Result<()> {
    // Step 1: Rebuild bootloader entries to drop GC roots for old generations.
    // Without this, the bootloader still references old system generations,
    // preventing nix-store --gc from reclaiming those store paths.
    output::header(&format!(
        "  [{pass}/{total}] Step 1/4: Rebuilding bootloader entries"
    ));
    CommandRunner::new("sudo")
        .args(["/run/current-system/bin/switch-to-configuration", "boot"])
        .run()?;

    // Step 2: Remove old generation profiles (keeps only the current one).
    output::header(&format!(
        "  [{pass}/{total}] Step 2/4: Removing old generations"
    ));
    CommandRunner::new("nh")
        .args(["clean", "all", "--keep", "0"])
        .run()?;

    // Step 3: Delete any remaining generations and garbage collect the store.
    // The -d flag ensures old generation symlinks are removed before GC runs.
    output::header(&format!(
        "  [{pass}/{total}] Step 3/4: Garbage collecting store"
    ));
    CommandRunner::new("nix-collect-garbage").arg("-d").run()?;

    // Step 4: Deduplicate the store by hard-linking identical files.
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
