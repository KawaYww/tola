mod build;
mod cli;
mod init;
mod deploy;
mod config;
mod serve;
mod utils;
mod watch;

use std::path::Path;
use anyhow::{bail, Result};
use build::build_site;
use clap::Parser;
use cli::{Cli, Commands};
use config::SiteConfig;
use deploy::deploy_site;
use init::new_site;
use serve::serve_site;

#[rustfmt::skip]
#[tokio::main]
async fn main() -> Result<()> {
    let cli: &'static Cli = Box::leak(Box::new(Cli::parse()));

    let config: &'static SiteConfig = {
        let root = cli.root.as_deref().unwrap_or(Path::new("./"));
        let config = root.join(&cli.config);
        let mut config =
            if config.exists() { SiteConfig::from_file(&config)? }
            else { SiteConfig::default() };
        config.update_with_cli(cli);

        let is_init_subcommand = matches!(cli.command, Commands::Init { .. });
        let config_exists = config.get_root().join(cli.config.as_path()).exists();
        match (is_init_subcommand, config_exists) {
            (true, false) => (),
            (true, true) => bail!("the config file exists, please remove the config file manually or init in other path"),
            (false, false) => bail!("the config file didn't exist"),
            (false, true) => config.validate(cli)?,
        }

        Box::leak(Box::new(config))
    };
       
    match cli.command {
        Commands::Init { .. } => new_site(config)?,
        Commands::Build { .. } => { build_site(config, config.build.clear)?; },
        Commands::Deploy { .. } => deploy_site(config)?,
        Commands::Serve { .. } => serve_site(config).await?
    };

    Ok(())
}
