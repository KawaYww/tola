mod builder;
mod cli;
mod log;
mod server;
mod utils;
mod watcher;

use anyhow::{anyhow, Result};
use builder::build_site;
use clap::Parser;
use cli::Cli;
use server::start_server;
use std::time::Duration;
use tokio::{signal, spawn, sync::oneshot};
use watcher::watch_for_changes_blocking;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let cli: &'static Cli = Box::leak(Box::new(cli));
    utils::check_typst_installed()?;

    if cli.command_is_built() {
        build_site(cli)?
    }
    

    if cli.command_is_serve() {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        std::thread::spawn(
            move || watch_for_changes_blocking(cli, shutdown_rx)
        );

        spawn({
            let timeout_secs = 2;
            let mut restart_flag = true;
            async move {
                while restart_flag {
                    match start_server(cli).await {
                        Ok(()) => restart_flag = false,
                        Err(e) => {
                            log!("error", "Failed to start server: {:?}", e);
                            for i in (0..=timeout_secs).rev() {
                                log!("tips", "Automatically trying to start it again in {} seconds", i);
                                tokio::time::sleep(Duration::from_secs(i)).await;
                            }
                        }
                    };
                }
            }
        });

        signal::ctrl_c().await?;
        shutdown_tx.send(()).map_err(|_| anyhow!("Failed to send shutdown message to watcher") )?;
    }

    Ok(())
}
