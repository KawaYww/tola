use crate::{
    cli::{self, Cli},
    log,
};
use anyhow::{bail, Context, Result};
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
};
use minify_html::{Cfg, minify};
use rayon::prelude::*;
use std::{
    env,
    ffi::OsStr,
    fs::{self, create_dir_all},
    io::stdout,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
};

pub fn check_typst_installed() -> Result<()> {
    Command::new("typst")
        .arg("--version")
        .output()
        .map(|_| ())
        .context("[Utils] Typst not found. Please install Typst first.")
}

pub fn _clear_screen() -> Result<()> {
    crossterm::execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))
        .context("[Utils] Failed to clear screen")
}

pub fn _copy_dir_recursively(src: &Path, dst: &Path) -> Result<()> {
    if !dst.exists() {
        create_dir_all(dst).context("[Utils] Failed to create destination directory")?;
    }

    for entry in fs::read_dir(src).context("[Utils] Failed to read source directory")? {
        let entry = entry.context("[Utils] Invalid directory entry")?;
        let entry_path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if entry_path.is_dir() {
            _copy_dir_recursively(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path).with_context(|| {
                format!("[Utils] Failed to copy {:?} to {:?}", entry_path, dest_path)
            })?;

            log!("assets", "{}", dest_path.display());
        }
    }

    Ok(())
}

pub fn process_files<P, F>(dir: &Path, cli: &Cli, p: &P, f: &F) -> Result<()>
where
    P: Fn(&OsStr) -> bool + Sync,
    F: Fn(&Path, &Cli) -> Result<()> + Sync,
{
    fs::read_dir(dir)?
        .collect::<Vec<_>>()
        .par_iter()
        .flatten()
        .try_for_each(|entry| {
            let path = entry.path();
            if path.is_dir() {
                process_files(&path, cli, p, f)
            } else if path.is_file() && path.extension().is_some_and(p) {
                f(&path, cli)
            } else {
                Ok(())
            }
        })
}

pub fn process_posts_in_parallel(files: &[PathBuf], cli: &Cli) -> Result<()> {
    files
        .par_iter()
        .try_for_each(|path| compile_post(path, cli))
}

pub fn copy_assets_in_parallel(files: &[PathBuf], cli: &Cli, should_wait_until_stable: bool) -> Result<()> {
    files.par_iter().try_for_each(|path| copy_asset(path, cli, should_wait_until_stable))
}

pub fn process_watched_files(files: &[PathBuf], cli: &Cli) -> Result<()> {
    let posts_files: Vec<_> = files
        .par_iter()
        .filter(|p| p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("typ"))
        .cloned()
        .collect();

    // println!("Before");

    let assets_files: Vec<_> = files
        .par_iter()
        // .inspect(|x| println!("{:?}", x))
        .filter(|p| {
            p.is_file()
                && p.strip_prefix(env::current_dir().unwrap())
                    .unwrap()
                    .starts_with(&cli.assets_dir)
        })
        .cloned()
        .collect();

    // println!("{:?}", assets_files);

    if !posts_files.is_empty() {
        process_posts_in_parallel(&posts_files, cli)?;
    }

    if !assets_files.is_empty() {
        copy_assets_in_parallel(&assets_files, cli, true)?;
    }

    Ok(())
}

pub fn compile_post(path: &Path, cli: &Cli) -> Result<()> {
    let path = path.canonicalize()?;
    let path = path.as_path();

    let current_dir = env::current_dir()?;
    let content_dir = current_dir.join(&cli.content_dir);
    let output_dir = current_dir.join(&cli.output_dir);

    // println!("{:?} {:?}", path, &content_dir);
    let relative_path = path
        .strip_prefix(&content_dir)?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid path"))?
        .strip_suffix(".typ")
        .ok_or_else(|| anyhow::anyhow!("Not a .typ file"))?;

    let output_path = output_dir.join(relative_path);
    create_dir_all(&output_path)?;

    let html_path = if path.file_name().is_some_and(|p| p == "home.typ") {
        current_dir.join(&cli.output_dir).join("index.html")
    } else {
        output_path.join("index.html")
    };

    let output = Command::new("typst")
        .args(["compile", "--features", "html", "--format", "html"])
        .arg("--font-path")
        .arg(&current_dir)
        .arg("--root")
        .arg(&current_dir)
        .arg(path)
        .arg(&html_path)
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to compile {}: {}", path.display(), error_msg);
    }

    if let Some(cli::Commands::Built { minify: true }) = cli.command {
        let html_content = fs::read_to_string(&html_path)?;
        let minified_content = minify(html_content.as_bytes(), &Cfg::new());
        let content = String::from_utf8_lossy(&minified_content).to_string();
        fs::write(&html_path, content)?;
    }

    log!("content", "{}", path.strip_prefix(&current_dir)?.display());

    Ok(())
}

pub fn copy_asset(path: &Path, cli: &Cli, should_wait_until_stable: bool) -> Result<()> {
    let path = path.canonicalize()?;
    let path = path.as_path();

    // println!("{:?}", path);

    let current_dir = env::current_dir()?;
    let assets_dir = current_dir.join(&cli.assets_dir);
    let output_dir = current_dir.join(&cli.output_dir).join(&cli.assets_dir);

    // println!("{:?} {:?}", path, &assets_dir);
    let relative_path = path
        .strip_prefix(&assets_dir)?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid path"))?;

    let output_path = output_dir.join(relative_path);

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    if output_path.exists() {
        fs::remove_file(&output_path)?;
    }

    if should_wait_until_stable {
        wait_until_stable(path, 5)?;
    }
    fs::copy(path, &output_path)?;

    log!("assets", "{}", output_path.strip_prefix(&current_dir)?.display());

    Ok(())
}

fn wait_until_stable(path: &Path, max_retries: usize) -> Result<()> {
    let mut last_size = fs::metadata(path)?.len();
    let mut retries = 0;
    let timeout = Duration::from_millis(50);
    
    while retries < max_retries {
        thread::sleep(timeout);
        let current_size = fs::metadata(path)?.len();
        if current_size == last_size {
            return Ok(());
        }
        last_size = current_size;
        retries += 1;
    }

    bail!("File did not stabilize after {} retries", max_retries);
}
