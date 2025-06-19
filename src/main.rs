mod builder;
mod cli;
mod log;
mod server;
mod utils;
mod watcher;

use anyhow::Result;
use builder::build_site;
use clap::Parser;
use cli::{Cli, Commands};
use server::start_server;
use std::time::Duration;
use tokio::{signal, spawn, sync::oneshot, task::spawn_blocking};
use watcher::watch_for_changes_blocking;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    utils::check_typst_installed()?;

    if let Some(command) = &cli.command {
        // utils::clear_screen()?;

        match command {
            Commands::Built { .. } => build_site(&cli)?,
            Commands::Serve { .. } => {
                let (shutdown_tx, shutdown_rx) = oneshot::channel();

                let server_handle = spawn({
                    let cli = cli.clone();
                    async move {
                        start_server(&cli).await.unwrap();
                    }
                });
                let watcher_handle = spawn_blocking({
                    let cli = cli.clone();
                    move || watch_for_changes_blocking(&cli, shutdown_rx)
                });

                signal::ctrl_c().await?;
                log!("server", "Initiating graceful shutdown...");
                let _ = shutdown_tx.send(());

                tokio::select! {
                    _ = server_handle => {},
                    _ = watcher_handle => {},
                    _ = tokio::time::sleep(Duration::from_secs(5)) => {
                        log!("error", "Timeout during graceful shutdown");
                    }
                }
            }
        }
    }

    Ok(())
}
