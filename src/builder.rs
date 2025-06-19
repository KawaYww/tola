use crate::{cli::Cli, log, utils::{self, compile_post, copy_asset}};
use anyhow::{Context, Result};
use std::fs;

pub fn build_site(cli: &Cli) -> Result<()> {
    // Clear output directory
    if cli.output_dir.exists() {
        fs::remove_dir_all(&cli.output_dir).with_context(|| {
            format!(
                "[Builder] Failed to clear output directory: {}",
                cli.output_dir.display()
            )
        })?;
    }

    // // Copy assets
    // utils::copy_dir_recursively(&cli.assets_dir, &cli.output_dir.join(&cli.assets_dir))
    //     .context("[Builder] Failed to copy assets")?;

    // Process all posts
    utils::process_files(&cli.content_dir,  cli, &|suffix| suffix == "typ", &compile_post)?;

    // // Copy assets
    utils::process_files(&cli.assets_dir,  cli, &|_| true, &|path, cli| copy_asset(path, cli, false))?;

    log!(
        "builder",
        "Successfully generated site in: {}",
        cli.output_dir.display()
    );

    Ok(())
}
