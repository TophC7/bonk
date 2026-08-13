//! Development shell command - wraps `nix develop`.

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use anyhow::Result;
use users::os::unix::UserExt;

use crate::cli::DevArgs;
use crate::exec::CommandRunner;
use crate::flake::resolve_flake_path;

/// Enter a flake development environment or run a command inside it.
///
/// `flake_path` overrides automatic flake discovery when present.
///
/// # Errors
///
/// Returns an error when no flake can be resolved or `nix` cannot be executed.
pub fn run(args: &DevArgs, flake_path: Option<&Path>) -> Result<()> {
    let flake = resolve_flake_path(flake_path)?;
    let target = args
        .shell
        .as_ref()
        .map_or_else(|| flake.clone(), |shell| format!("{flake}#{shell}"));
    let mut runner = CommandRunner::new("nix")
        .arg("develop")
        .arg(target)
        .arg_if(args.impure, "--impure");

    if !args.command.is_empty() {
        runner = runner
            .arg("--command")
            .args(args.command.iter().map(String::as_str));
    } else if !args.bash {
        if let Some(shell) = user_shell() {
            let mut shell_environment = OsString::from("SHELL=");
            shell_environment.push(&shell);
            runner = runner
                .arg("--command")
                .arg("env")
                .arg(shell_environment)
                .arg(shell.into_os_string());
        }
    }

    runner.exec()
}

fn user_shell() -> Option<PathBuf> {
    select_user_shell(env::var_os("SHELL"), || {
        users::get_user_by_uid(users::get_current_uid()).map(|user| user.shell().to_path_buf())
    })
}

fn select_user_shell(
    environment_shell: Option<OsString>,
    account_shell: impl FnOnce() -> Option<PathBuf>,
) -> Option<PathBuf> {
    environment_shell
        .filter(|shell| !shell.is_empty())
        .map(PathBuf::from)
        .filter(|shell| shell.is_file())
        .or_else(|| {
            account_shell().filter(|shell| !shell.as_os_str().is_empty() && shell.is_file())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_shell_takes_precedence() {
        let shell = select_user_shell(Some(OsString::from("/bin/sh")), || {
            Some(PathBuf::from("/definitely/missing"))
        });
        assert_eq!(shell, Some(PathBuf::from("/bin/sh")));
    }

    #[test]
    fn account_shell_is_the_fallback() {
        let shell = select_user_shell(None, || Some(PathBuf::from("/bin/sh")));
        assert_eq!(shell, Some(PathBuf::from("/bin/sh")));
    }

    #[test]
    fn invalid_environment_shell_uses_account_shell() {
        let shell = select_user_shell(Some(OsString::from("/definitely/missing")), || {
            Some(PathBuf::from("/bin/sh"))
        });
        assert_eq!(shell, Some(PathBuf::from("/bin/sh")));
    }

    #[test]
    fn missing_shells_use_plain_nix_develop() {
        assert_eq!(select_user_shell(None, || None), None);
    }
}
