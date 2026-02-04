//! Store info command - shows store statistics.

use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::cli::store::InfoArgs;
use crate::exec::CommandRunner;
use crate::output;

/// Count entries in a directory.
fn count_dir_entries(path: &Path) -> usize {
    fs::read_dir(path)
        .map(|entries| entries.count())
        .unwrap_or(0)
}

/// Count entries matching prefix and suffix in a directory.
fn count_matching_entries(dir: &Path, prefix: &str, suffix: &str) -> usize {
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    name_str.starts_with(prefix) && name_str.ends_with(suffix)
                })
                .count()
        })
        .unwrap_or(0)
}

/// Count total entries across all subdirectories.
fn count_nested_entries(base: &Path) -> usize {
    fs::read_dir(base)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_dir())
                .map(|user_dir| count_dir_entries(&user_dir.path()))
                .sum()
        })
        .unwrap_or(0)
}

/// Execute the store info command.
pub fn run(args: &InfoArgs) -> Result<()> {
    output::header("Nix Store Information");

    output::info("Store size:");
    CommandRunner::new("du")
        .args(["-sh", "/nix/store"])
        .show_command(false)
        .run()?;

    output::info("Store path count:");
    let store_path = Path::new("/nix/store");
    let path_count = count_dir_entries(store_path);
    println!("  {} paths", path_count);

    println!();
    output::info("System generations:");
    let profiles_path = Path::new("/nix/var/nix/profiles");
    let system_gen_count = count_matching_entries(profiles_path, "system-", "-link");
    if system_gen_count > 0 {
        println!("  {} system generations", system_gen_count);
    } else {
        println!("  (could not determine)");
    }

    let per_user_path = Path::new("/nix/var/nix/profiles/per-user");
    let user_gen_count = count_nested_entries(per_user_path);
    if user_gen_count > 0 {
        println!("  {} user profile generations", user_gen_count);
    } else {
        println!("  (could not determine)");
    }

    if args.detailed {
        println!();
        output::info("Detailed breakdown (this may take a while)...");

        println!();
        output::status("Live paths:");
        let live_count = CommandRunner::new("nix-store")
            .args(["--gc", "--print-live"])
            .show_command(false)
            .inherit_stdio(false)
            .run_output()
            .map(|(out, _)| out.lines().count())
            .unwrap_or(0);
        if live_count > 0 {
            println!("  {} live paths", live_count);
        } else {
            println!("  ?");
        }

        output::status("Dead paths (can be garbage collected):");
        let dead_count = CommandRunner::new("nix-store")
            .args(["--gc", "--print-dead"])
            .show_command(false)
            .inherit_stdio(false)
            .run_output()
            .map(|(out, _)| out.lines().count())
            .unwrap_or(0);
        if dead_count > 0 {
            println!("  {} dead paths", dead_count);
        } else {
            println!("  ?");
        }
    }

    Ok(())
}
