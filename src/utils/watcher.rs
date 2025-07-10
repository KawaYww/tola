use std::{fs, path::Path, thread, time::Duration};
use anyhow::{bail, Result};

pub fn wait_until_stable(path: &Path, max_retries: usize) -> Result<()> {
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

