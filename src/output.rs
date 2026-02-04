//! Colored terminal output helpers.

use owo_colors::OwoColorize;

pub fn info(message: &str) {
    println!("{} {}", "::".blue().bold(), message);
}

pub fn success(message: &str) {
    println!("{} {}", "::".green().bold(), message);
}

pub fn warn(message: &str) {
    println!("{} {}", "::".yellow().bold(), message);
}

pub fn show_cmd(cmd: &str) {
    println!("{} {}", ">".dimmed(), cmd.dimmed());
}

pub fn status(message: &str) {
    println!("{} {}", "->".cyan(), message);
}

pub fn header(title: &str) {
    println!("\n{}", title.bold());
    println!("{}", "=".repeat(title.len()).dimmed());
}

#[allow(dead_code)]
pub fn kv(key: &str, value: &str) {
    println!("  {}: {}", key.dimmed(), value);
}
