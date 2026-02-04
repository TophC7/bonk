//! Try command - wraps `nix shell`.

use anyhow::Result;

use crate::cli::TryArgs;
use crate::exec::CommandRunner;
use crate::output;

/// Execute the try command.
pub fn run(args: &TryArgs) -> Result<()> {
    let packages: Vec<String> = args
        .packages
        .iter()
        .map(|pkg| {
            if pkg.contains('#') || pkg.starts_with('.') || pkg.starts_with('/') {
                pkg.clone()
            } else {
                format!("nixpkgs#{}", pkg)
            }
        })
        .collect();

    let pkg_display: Vec<&str> = args.packages.iter().map(String::as_str).collect();
    output::info(&format!("Starting shell with: {}", pkg_display.join(", ")));

    let mut runner = CommandRunner::new("nix").arg("shell");

    for pkg in &packages {
        runner = runner.arg(pkg);
    }

    runner = runner.arg_if(args.pure, "--ignore-environment");

    if !args.cmd.is_empty() {
        runner = runner.arg("--command");
        for arg in &args.cmd {
            runner = runner.arg(arg);
        }
    }

    runner.run()
}
