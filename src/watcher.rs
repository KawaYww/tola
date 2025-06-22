use crate::{cli::Commands, log, utils};
use anyhow::{Context, Result};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use std::{mpsc, time::{Duration, Instant}};
use tokio::sync::oneshot;

use super::cli::Cli;

pub fn watch_for_changes_blocking(cli: &'static Cli, mut shutdown_rx: oneshot::Receiver<()>) -> Result<()> {
    if let Some(Commands::Serve { watch: false, .. }) = cli.command {
        return Ok(());
    }
    
    let (tx, rx) = mpsc::channel();
    let mut watcher =
        notify::recommended_watcher(tx).context("[watcher] Failed to create file watcher")?;

    watcher
        .watch(&cli.content_dir, RecursiveMode::Recursive)
        .context(format!(
            "[watcher] Failed to watch directory: {}",
            cli.content_dir.display()
        ))?;

    watcher
        .watch(&cli.assets_dir, RecursiveMode::Recursive)
        .context(format!(
            "[watcher] Failed to watch directory: {}",
            cli.assets_dir.display()
        ))?;

    let mut last_event_time = Instant::now();
    let debounce_duration = Duration::from_millis(50);

    log!(
        "watcher",
        "Watching for changes in {}...",
        cli.content_dir.display()
    );

    for res in rx {
        match res {
            Ok(event) => {
                if should_process_event(&event) && last_event_time.elapsed() > debounce_duration {
                    last_event_time = Instant::now();
                    std::thread::spawn(move || {

                        match handle_files(&event.paths, cli) {
                            Ok(()) => (),
                            Err(e) => log!("watcher", "Error: {:?}", e),
                        }

                    });
                }
            },
            Err(e) => {
                log!("watcher", "Error: {:?}", e);
            },
        };

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
    // log!("watcher", "Detected changes in: {:?}", paths);
    utils::process_watched_files(paths, cli).context("Failed to process changed files")
}
