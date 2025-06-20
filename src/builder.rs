use crate::{cli::Cli, log, utils::{self, compile_post, copy_asset}};
use anyhow::{anyhow, Context, Result};
use std::{fs, thread};

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

    thread::scope(|s| {
        // Process all posts
        let posts_handle = s.spawn(|| {
            utils::process_files(&cli.content_dir,  cli, &|suffix| suffix == "typ", &compile_post)
        });

        // Copy assets
        let assets_handle = s.spawn(|| {
            utils::process_files(&cli.assets_dir,  cli, &|_| true, &|path, cli| copy_asset(path, cli, false)).context("")
        });

        posts_handle.join().map_err(|e| anyhow!("{:?}", e))??;
        assets_handle.join().map_err(|e| anyhow!("{:?}", e))??;

        log!(
            "builder",
            "Successfully generated site in: {}",
            cli.output_dir.display()
        );

        Ok(())
    })
}
