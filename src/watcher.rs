use crate::{cli::Commands, log, utils};
use anyhow::{Context, Result};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::{
    path::PathBuf,
    sync::mpsc,
    time::{Duration, Instant},
};
use tokio::sync::oneshot;

use super::cli::Cli;

pub fn watch_for_changes_blocking(cli: &Cli, mut shutdown_rx: oneshot::Receiver<()>) -> Result<()> {
    if let Some(Commands::Serve { watch: false, .. }) = cli.command {
        return Ok(());
    }
    
    let (tx, rx) = mpsc::channel();
    let mut watcher =
        notify::recommended_watcher(tx).context("[Watcher] Failed to create file watcher")?;

    watcher
        .watch(&cli.content_dir, RecursiveMode::Recursive)
        .context(format!(
            "[Watcher] Failed to watch directory: {}",
            cli.content_dir.display()
        ))?;
    watcher
        .watch(&cli.assets_dir, RecursiveMode::Recursive)
        .context(format!(
            "[Watcher] Failed to watch directory: {}",
            cli.assets_dir.display()
        ))?;

    let mut last_event_time = Instant::now();
    let debounce_duration = Duration::from_millis(500);
    let poll_timeout = Duration::from_millis(100);

    log!(
        "watcher",
        "Watching for changes in {}...",
        cli.content_dir.display()
    );

    loop {
        match rx.recv_timeout(poll_timeout) {
            Ok(Ok(event)) => {
                if should_process_event(&event) && last_event_time.elapsed() >= debounce_duration {
                    last_event_time = Instant::now();
                    match handle_files(&event.paths, cli) {
                        Ok(()) => (),
                        Err(e) => log!("watcher", "Error: {:?}", e),
                    };
                }
            }
            Ok(Err(e)) => {
                log!("watcher", "Error: {:?}", e);
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                log!("watcher", "Channel disconnected");
                break;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Do not need show message here
            }
        }

        if shutdown_rx.try_recv().is_ok() {
            log!("watcher", "Received shutdown signal");
            break;
        }
    }

    Ok(())
}

fn should_process_event(event: &Event) -> bool {
    matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_))
}

fn handle_files(paths: &[PathBuf], cli: &Cli) -> Result<()> {
    log!("watcher", "Detected changes in: {:?}", paths);
    utils::process_watched_files(paths, cli).context("[Watcher] Failed to process changed files")
}
