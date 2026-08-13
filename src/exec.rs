//! External command execution utilities.

use std::ffi::{OsStr, OsString};
use std::os::unix::process::CommandExt;
use std::process::{Command, ExitStatus, Stdio};

use anyhow::{Context, Result};

use crate::output;

/// Builder for executing external commands.
pub struct CommandRunner {
    program: String,
    args: Vec<OsString>,
    envs: Vec<(String, String)>,
    show_command: bool,
    inherit_stdio: bool,
}

impl CommandRunner {
    #[must_use]
    pub fn new(program: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
            envs: Vec::new(),
            show_command: true,
            inherit_stdio: true,
        }
    }

    /// Set an environment variable for the spawned process.
    #[must_use]
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.envs.push((key.into(), value.into()));
        self
    }

    #[must_use]
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    #[must_use]
    pub fn arg(mut self, arg: impl Into<OsString>) -> Self {
        self.args.push(arg.into());
        self
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn args_if<I, S>(self, condition: bool, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        if condition {
            self.args(args)
        } else {
            self
        }
    }

    #[must_use]
    pub fn arg_if(self, condition: bool, arg: impl Into<OsString>) -> Self {
        if condition {
            self.arg(arg)
        } else {
            self
        }
    }

    #[must_use]
    pub fn show_command(mut self, show: bool) -> Self {
        self.show_command = show;
        self
    }

    #[must_use]
    pub fn inherit_stdio(mut self, inherit: bool) -> Self {
        self.inherit_stdio = inherit;
        self
    }

    fn command_string(&self) -> String {
        std::iter::once(OsStr::new(&self.program))
            .chain(self.args.iter().map(OsString::as_os_str))
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn run(self) -> Result<()> {
        let status = self.run_status()?;
        if status.success() {
            Ok(())
        } else {
            let code = status.code().unwrap_or(-1);
            anyhow::bail!("command failed with exit code {}", code)
        }
    }

    pub fn run_status(self) -> Result<ExitStatus> {
        if self.show_command {
            output::show_cmd(&self.command_string());
        }

        self.command(self.inherit_stdio)
            .status()
            .with_context(|| format!("failed to execute '{}'", self.program))
    }

    /// Replace Bonk with the configured command.
    ///
    /// This preserves the command's exit status and signal behavior.
    ///
    /// # Errors
    ///
    /// Returns an error only when the operating system cannot execute the command.
    pub fn exec(self) -> Result<()> {
        if self.show_command {
            output::show_cmd(&self.command_string());
        }

        let error = self.command(self.inherit_stdio).exec();
        Err(error).with_context(|| format!("failed to execute '{}'", self.program))
    }

    pub fn run_output(self) -> Result<(String, String)> {
        if self.show_command {
            output::show_cmd(&self.command_string());
        }

        let output = self
            .command(false)
            .output()
            .with_context(|| format!("failed to execute '{}'", self.program))?;

        if !output.status.success() {
            let code = output.status.code().unwrap_or(-1);
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("command failed with exit code {}: {}", code, stderr.trim());
        }

        let stdout =
            String::from_utf8(output.stdout).context("command stdout contained invalid UTF-8")?;
        let stderr =
            String::from_utf8(output.stderr).context("command stderr contained invalid UTF-8")?;

        Ok((stdout, stderr))
    }

    fn command(&self, inherit_stdio: bool) -> Command {
        let mut command = Command::new(&self.program);
        command.args(&self.args);
        command.envs(self.envs.iter().map(|(key, value)| (key, value)));

        if inherit_stdio {
            command
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }

        command
    }
}

#[allow(dead_code)]
pub fn program_exists(program: &str) -> bool {
    match Command::new("which")
        .arg(program)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Ok(status) => status.success(),
        Err(e) => {
            // Log the error rather than silently swallowing it.
            // This helps diagnose issues like `which` not being available.
            tracing::warn!("Failed to check if '{}' exists: {}", program, e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_string() {
        let runner = CommandRunner::new("nh")
            .args(["os", "switch", "."])
            .arg("-H")
            .arg("rune");
        assert_eq!(runner.command_string(), "nh os switch . -H rune");
    }

    #[test]
    fn test_args_if_true() {
        let runner = CommandRunner::new("test").args_if(true, ["--flag"]);
        assert_eq!(runner.args, vec!["--flag"]);
    }

    #[test]
    fn test_args_if_false() {
        let runner = CommandRunner::new("test").args_if(false, ["--flag"]);
        assert!(runner.args.is_empty());
    }

    #[test]
    fn test_program_exists_false() {
        assert!(!program_exists("nonexistent_program_xyz_123"));
    }
}
