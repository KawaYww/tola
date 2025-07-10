use std::process::Command;
use anyhow::{Context, Result};

pub fn check_typst_installed() -> Result<()> {
    Command::new("typst")
        .arg("--version")
        .output()
        .map(|_| ())
        .context("[Utils] Typst not found. Please install Typst first.")
}


